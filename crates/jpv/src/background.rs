use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use anyhow::{anyhow, bail, Context, Result};
use flate2::read::GzDecoder;
use lib::config::{Config, IndexKind};
use lib::database::{self, Database, Input};
use lib::reporter::{Reporter, TracingReporter};
use lib::token::Token;
use lib::{api, data, Dirs};
use reqwest::Method;
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

use crate::reporter::EventsReporter;
use crate::system::SystemEvents;
use crate::tasks::Tasks;
use crate::Args;

/// The user agent used by jpv.
const USER_AGENT: &str = concat!("jpv/", env!("CARGO_PKG_VERSION"));

const JMDICT_URL: &str = "http://ftp.edrdg.org/pub/Nihongo/JMdict_e_examp.gz";
const KANJIDIC2_URL: &str = "http://ftp.edrdg.org/pub/Nihongo/kanjidic2.xml.gz";
const JMNEDICT_URL: &str = "http://ftp.edrdg.org/pub/Nihongo/JMnedict.xml.gz";

pub(crate) struct BackgroundInner {
    config: Config,
    database: Database,
    pub(crate) log: Vec<api::LogEntry>,
}

/// Events emitted by modifying the background service.
pub enum BackgroundEvent {
    /// Save configuration file.
    SaveConfig(Config, oneshot::Sender<()>),
    /// Force a database rebuild.
    Rebuild(bool),
}

#[derive(Clone)]
pub struct Background {
    dirs: Arc<Dirs>,
    channel: UnboundedSender<BackgroundEvent>,
    inner: Arc<RwLock<BackgroundInner>>,
}

impl Background {
    pub(crate) fn new(
        dirs: Dirs,
        channel: UnboundedSender<BackgroundEvent>,
        config: Config,
        database: Database,
    ) -> Self {
        Self {
            dirs: Arc::new(dirs),
            channel,
            inner: Arc::new(RwLock::new(BackgroundInner {
                config,
                database,
                log: Vec::new(),
            })),
        }
    }

    /// Get the current log backfill.
    pub(crate) fn log(&self) -> Vec<api::LogEntry> {
        self.inner.read().unwrap().log.clone()
    }

    /// Update current configuration.
    pub(crate) async fn update_config(&self, config: Config) -> bool {
        let (sender, receiver) = oneshot::channel();

        let _ = self
            .channel
            .send(BackgroundEvent::SaveConfig(config.clone(), sender));

        if receiver.await.is_err() {
            return false;
        }

        self.inner.write().unwrap().config = config;
        true
    }

    /// Trigger a rebuild.
    pub(crate) async fn rebuild(&self) {
        let _ = self.channel.send(BackgroundEvent::Rebuild(false));
    }

    /// Access current configuration.
    pub(crate) fn config(&self) -> Config {
        self.inner.read().unwrap().config.clone()
    }

    /// Access the database currently in use.
    pub(crate) fn database(&self) -> Database {
        self.inner.read().unwrap().database.clone()
    }

    /// Handle a background event.
    pub(crate) async fn handle_event(
        &self,
        event: BackgroundEvent,
        args: &Args,
        tasks: &mut Tasks,
        system_events: &SystemEvents,
    ) -> Result<()> {
        match event {
            BackgroundEvent::SaveConfig(config, callback) => {
                let dirs = self.dirs.clone();
                let path = dirs.config_path();
                ensure_parent_dir(&path).await?;

                let new_config = config.clone();

                let task = tokio::task::spawn_blocking(move || {
                    let config = lib::toml::to_string_pretty(&config)?;

                    let mut tempfile = NamedTempFile::new_in(dirs.config_dir())?;
                    std::io::copy(&mut config.as_bytes(), &mut tempfile)?;
                    tempfile.persist(&path)?;
                    tracing::info!("Wrote new configuration to {}", path.display());
                    Ok::<_, anyhow::Error>(())
                });

                task.await??;

                let indexes = data::open_from_args(&args.index[..], &self.dirs)?;
                let db = lib::database::Database::open(indexes, &new_config)?;
                self.inner.write().unwrap().database = db;
                let _ = callback.send(());
            }
            BackgroundEvent::Rebuild(force) => {
                let config = self.config();
                let dirs = self.dirs.clone();
                let inner = self.inner.clone();
                let index = args.index.clone();

                let reporter = Arc::new(EventsReporter {
                    parent: TracingReporter,
                    inner: inner.clone(),
                    system_events: system_events.clone(),
                });

                let Some((shutdown, completion)) = tasks.unique_task("rebuild database") else {
                    return Ok(());
                };

                tokio::spawn(async move {
                    // Capture the completion handler so that it is dropped with the task.
                    let _completion = completion;
                    let to_download = config_to_download(&config, &dirs, Default::default());
                    let result = build(reporter, shutdown, &dirs, &to_download, force).await;

                    if !result? {
                        return Ok(());
                    }

                    let mut inner = inner.write().unwrap();
                    let indexes = data::open_from_args(&index[..], &dirs)?;
                    let db = lib::database::Database::open(indexes, &inner.config)?;
                    inner.database = db;
                    Ok::<_, anyhow::Error>(())
                });
            }
        }

        Ok(())
    }
}

/// Path and url to download.
pub struct ToDownload {
    pub name: String,
    pub url: String,
    pub url_name: String,
    pub index_path: Box<Path>,
    pub path: Option<Box<Path>>,
    pub kind: IndexKind,
}

/// Download override paths.
#[derive(Default)]
pub struct DownloadOverrides<'a> {
    overrides: HashMap<IndexKind, &'a Path>,
}

impl<'a> DownloadOverrides<'a> {
    /// Insert a download override.
    pub fn insert(&mut self, kind: IndexKind, path: &'a Path) {
        self.overrides.insert(kind, path);
    }

    fn get(&self, kind: IndexKind) -> Option<&'a Path> {
        self.overrides.get(&kind).copied()
    }
}

/// Convert configuration into indexes that should be downloaded and built.
pub fn config_to_download(
    config: &Config,
    dirs: &Dirs,
    overrides: DownloadOverrides<'_>,
) -> Vec<ToDownload> {
    let mut downloads = Vec::new();

    for &index in &config.enabled {
        let path = overrides.get(index).map(|p| p.into());

        let download = match index {
            IndexKind::Jmdict => ToDownload {
                name: index.name().into(),
                url: JMDICT_URL.into(),
                url_name: "JMdict_e_examp.gz".into(),
                index_path: dirs.index_path(index.name()).into(),
                path,
                kind: index,
            },
            IndexKind::Kanjidic2 => ToDownload {
                name: index.name().into(),
                url: KANJIDIC2_URL.into(),
                url_name: "kanjidic2.xml.gz".into(),
                index_path: dirs.index_path(index.name()).into(),
                path,
                kind: index,
            },
            IndexKind::Jmnedict => ToDownload {
                name: index.name().into(),
                url: JMNEDICT_URL.into(),
                url_name: "jmnedict.xml.gz".into(),
                index_path: dirs.index_path(index.name()).into(),
                path,
                kind: index,
            },
        };

        downloads.push(download);
    }

    downloads
}

/// Build the database in the background.
#[must_use = "Must check that the build completed before proceeding"]
pub(crate) async fn build(
    reporter: Arc<dyn Reporter>,
    shutdown: oneshot::Receiver<()>,
    dirs: &Dirs,
    to_download: &[ToDownload],
    force: bool,
) -> Result<bool> {
    let shutdown_token = Token::new();

    let future = async {
        for download in to_download {
            if shutdown_token.is_set() {
                return Ok(false);
            }

            ensure_parent_dir(&download.index_path).await?;

            // SAFETY: We are the only ones calling this function now.
            let result = lib::data::open(&download.index_path);

            match result {
                Ok(data) => match database::Index::open(data) {
                    Ok(..) => {
                        if !force {
                            lib::report_info!(
                                reporter,
                                "Dictionary already exists at {}",
                                download.index_path.display()
                            );
                            continue;
                        } else {
                            lib::report_info!(
                                reporter,
                                "Dictionary already exists at {} (forcing rebuild)",
                                download.index_path.display()
                            );
                        }
                    }
                    Err(error) => {
                        lib::report_warn!(
                            reporter,
                            "Rebuilding since exists, but could not open: {error}: {}",
                            download.index_path.display()
                        );
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    bail!(e)
                }
            }

            let future = async {
                let (path, data) = read_or_download(
                    &*reporter,
                    download.path.as_deref(),
                    dirs,
                    &download.url_name,
                    &download.url,
                )
                .await
                .context("loading JMDICT")?;

                lib::report_info!(
                    reporter,
                    "Loading `{}` from {}",
                    download.name,
                    path.display()
                );

                let start = Instant::now();
                let kind = download.kind;
                let name = download.name.clone();

                let data = tokio::task::spawn_blocking({
                    let reporter = reporter.clone();
                    let shutdown_token = shutdown_token.clone();
                    move || {
                        let input = match kind {
                            IndexKind::Jmdict => Input::Jmdict(&data[..]),
                            IndexKind::Kanjidic2 => Input::Kanjidic(&data[..]),
                            IndexKind::Jmnedict => Input::Jmnedict(&data[..]),
                        };

                        database::build(&*reporter, &shutdown_token, &name, input)
                    }
                });

                let data = data.await??;

                let duration = Instant::now().duration_since(start);

                fs::write(&download.index_path, data.as_slice())
                    .await
                    .with_context(|| anyhow!("{}", download.index_path.display()))?;

                lib::report_info!(
                    reporter,
                    "Took {duration:?} to build index at {}",
                    download.index_path.display()
                );

                Ok::<_, anyhow::Error>(())
            };

            if let Err(error) = future.await {
                lib::report_error!(reporter, "Error building `{}`", download.name);

                for error in error.chain() {
                    lib::report_error!(reporter, "Caused by: {error}");
                }
            }
        }

        Ok(true)
    };

    let mut future = pin!(future);

    let completed = tokio::select! {
        result = future.as_mut() => {
            result?
        }
        _ = shutdown => {
            shutdown_token.set();
            future.await?
        }
    };

    Ok(completed)
}

async fn read_or_download(
    reporter: &dyn Reporter,
    path: Option<&Path>,
    dirs: &Dirs,
    name: &str,
    url: &str,
) -> Result<(PathBuf, String), anyhow::Error> {
    let (path, bytes) = match path {
        Some(path) => (path.to_owned(), fs::read(path).await?),
        None => {
            let path = dirs.cache_dir(name);

            let bytes = if !path.is_file() {
                download(reporter, url, &path)
                    .await
                    .with_context(|| anyhow!("Downloading {url} to {}", path.display()))?
            } else {
                fs::read(&path).await?
            };

            (path, bytes)
        }
    };

    let mut input = GzDecoder::new(&bytes[..]);
    let mut string = String::new();
    input
        .read_to_string(&mut string)
        .with_context(|| path.display().to_string())?;
    Ok((path, string))
}

async fn download(reporter: &dyn Reporter, url: &str, path: &Path) -> Result<Vec<u8>> {
    lib::report_info!(reporter, "Downloading {url} to {}", path.display());

    ensure_parent_dir(path).await?;

    let client = reqwest::ClientBuilder::new().build()?;

    let request = client
        .request(Method::GET, url)
        .header("User-Agent", USER_AGENT)
        .build()?;

    let mut response = client.execute(request).await?;

    let mut f = File::create(path).await?;
    let mut data = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        f.write_all(chunk.as_ref()).await?;
        data.extend_from_slice(chunk.as_ref());
    }

    Ok(data)
}

async fn ensure_parent_dir(path: &Path) -> Result<&Path> {
    let Some(parent) = path.parent() else {
        bail!("Missing parent directory for {}", path.display());
    };

    let is_dir = match fs::metadata(parent).await {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
        Ok(metadata) if !metadata.is_dir() => false,
        _ => true,
    };

    if !is_dir {
        fs::create_dir_all(parent).await?;
    }

    Ok(parent)
}
