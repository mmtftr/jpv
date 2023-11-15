mod godan;
#[macro_use]
mod macros;

pub use self::conjugate::{conjugate, Kind, Reading};
mod conjugate;

use std::fmt;
use std::ops::{BitAndAssign, BitOr};
use std::{collections::BTreeMap, ops::BitXor};

use fixed_map::raw::RawStorage;
use fixed_map::{Key, Set};
use musli::{Decode, Encode};
use musli_zerocopy::buf::{Padder, Validator};
use musli_zerocopy::{ByteOrder, ZeroCopy};
use serde::{Deserialize, Serialize};

use crate::kana::{Fragments, Full, OwnedFull};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Encode,
    Decode,
    Serialize,
    Deserialize,
    Key,
)]
#[key(bitset = 8)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    /// The stem of the word.
    Stem,
    /// Te-form.
    Te,
    /// Te-iru or progressive form.
    TeIru,
    /// Te-aru or resulting form.
    TeAru,
    /// Te-iku form.
    TeIku,
    /// te-shimau form
    TeShimau,
    /// chau form
    Chau,
    /// te-kuru form
    TeKuru,
    /// te-oku form
    TeOku,
    Command,
    Hypothetical,
    /// Alternate negative hypoethical form.
    Kya,
    Conditional,
    Passive,
    Potential,
    /// Volitional / Presumptive
    Volitional,
    /// To do, to let, to allow.
    Causative,
    /// Desire form.
    Tai,
    /// Negative tense.
    Negative,
    /// Past tense.
    Past,
    /// Conversational form.
    Conversation,
    /// Alternate shortened form, when available.
    Short,
    /// Alternate form using kudasai.
    TeKudasai,
    /// Alternate form using darou.
    Darou,
    /// Alternate command form using yo.
    Yo,
    /// Honorific speech.
    Honorific,
}

impl Form {
    pub const ALL: [Form; 26] = [
        Form::Stem,
        Form::Short,
        Form::Causative,
        Form::Chau,
        Form::Command,
        Form::Conditional,
        Form::Conversation,
        Form::Hypothetical,
        Form::Kya,
        Form::Negative,
        Form::Passive,
        Form::Past,
        Form::Potential,
        Form::Tai,
        Form::Te,
        Form::TeAru,
        Form::TeIku,
        Form::TeIru,
        Form::TeKuru,
        Form::TeOku,
        Form::TeShimau,
        Form::Volitional,
        Form::TeKudasai,
        Form::Darou,
        Form::Yo,
        Form::Honorific,
    ];

    /// Longer title for the form.
    pub fn title(&self) -> &'static str {
        match self {
            Form::Stem => "stem, or infinite form",
            Form::Short => "alternate shortened form",
            Form::Causative => "causative, make ~ do something, let / allow ~",
            Form::Chau => "~ちゃう, to do something by accident, to finish completely",
            Form::Command => "command forms, よ / なさい / ください",
            Form::Conditional => "conditional, if ~, when ~",
            Form::Conversation => "conversational / colloquial",
            Form::Hypothetical => "hypothetical, if ~",
            Form::Kya => "~きゃ, alternative hypothetical negative, if not ~",
            Form::Negative => "not doing ~, the absense of ~",
            Form::Passive => "passive, ~ was done to someone or something",
            Form::Past => "過去形 (かこけい) past tense",
            Form::Potential => "potential, can do ~",
            Form::Tai => "~たい, used to express desire",
            Form::Te => "~te form, by itself acts as a command",
            Form::TeAru => "~てある, resulting, is / has been done",
            Form::TeIku => "~ていく, starting, to start, to continue, to go on",
            Form::TeIru => {
                "~ている, progressive, shows that something is currently happening or ongoing"
            }
            Form::TeKuru => "~てくる, to do .. and come back, to become, to continue, to start ~",
            Form::TeOku => "~ておく, to do something in advance",
            Form::TeShimau => "~てしまう, to do something by accident, to finish completely",
            Form::Volitional => "volitional / presumptive, let's do ~",
            Form::TeKudasai => "alternate form using ~te kudasai",
            Form::Darou => "~だろう, alternate form",
            Form::Yo => "~よ, alternate command form using",
            Form::Honorific => "敬語 (ていご) honorific speech",
        }
    }

    /// Describe the form.
    pub fn describe(&self) -> &'static str {
        match self {
            Form::Stem => "stem",
            Form::Short => "short",
            Form::Causative => "caus",
            Form::Chau => "~ちゃう",
            Form::Command => "cmd",
            Form::Conditional => "cond",
            Form::Conversation => "clq",
            Form::Hypothetical => "hyp",
            Form::Kya => "~きゃ",
            Form::Negative => "not",
            Form::Passive => "passive",
            Form::Past => "past",
            Form::Potential => "pot",
            Form::Tai => "~たい",
            Form::Te => "~て",
            Form::TeAru => "~てある",
            Form::TeIku => "~ていく",
            Form::TeIru => "~ている",
            Form::TeKuru => "~てくる",
            Form::TeOku => "~ておく",
            Form::TeShimau => "~てしまう",
            Form::Volitional => "vol",
            Form::TeKudasai => "~てください",
            Form::Darou => "~だろう",
            Form::Yo => "~よ",
            Form::Honorific => "敬語",
        }
    }
}

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Encode,
    Decode,
)]
#[serde(transparent)]
#[musli(transparent)]
#[repr(transparent)]
pub struct Inflection {
    #[musli(with = crate::musli::set::<_>)]
    form: Set<Form>,
}

unsafe impl ZeroCopy for Inflection
where
    <<Form as Key>::SetStorage as RawStorage>::Value: ZeroCopy,
{
    const ANY_BITS: bool = false;
    const PADDED: bool = false;
    const CAN_SWAP_BYTES: bool = <<Form as Key>::SetStorage as RawStorage>::Value::CAN_SWAP_BYTES;

    #[inline]
    unsafe fn validate(v: &mut Validator<'_, Self>) -> Result<(), musli_zerocopy::Error> {
        <<Form as Key>::SetStorage as RawStorage>::Value::validate(v.transparent())
    }

    #[inline]
    unsafe fn pad(p: &mut Padder<'_, Self>) {
        <<Form as Key>::SetStorage as RawStorage>::Value::pad(p.transparent())
    }

    #[inline]
    fn swap_bytes<E: ByteOrder>(self) -> Self {
        let form = <<Form as Key>::SetStorage as RawStorage>::Value::swap_bytes(self.form.as_raw());

        Inflection {
            form: Set::from_raw(form),
        }
    }
}

impl Inflection {
    // Macro support.
    #[doc(hidden)]
    pub fn new(form: Set<Form>) -> Self {
        Self { form }
    }

    // Construct an inflection with all options set.
    pub fn all() -> Self {
        let mut form = Set::new();

        for f in Form::ALL {
            form.insert(f);
        }

        Self { form }
    }

    /// Toggle the given form.
    pub fn toggle(&mut self, form: Form) {
        if self.form.contains(form) {
            self.form.remove(form);
        } else {
            self.form.insert(form);
        }
    }

    /// Test if inflection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.form.is_empty()
    }

    /// Test if inflection contains the given form.
    #[inline]
    pub fn contains(&self, f: Form) -> bool {
        self.form.contains(f)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Form> {
        self.form.iter()
    }
}

impl fmt::Debug for Inflection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.form.fmt(f)
    }
}

impl BitOr for Inflection {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            form: Set::from_raw(self.form.as_raw() | rhs.form.as_raw()),
        }
    }
}

impl BitXor for Inflection {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            form: Set::from_raw(self.form.as_raw() ^ rhs.form.as_raw()),
        }
    }
}

impl BitAndAssign for Inflection {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.form = Set::from_raw(self.form.as_raw() & rhs.form.as_raw());
    }
}

/// A collection of inflections.
#[borrowme::borrowme]
pub struct Inflections<'a> {
    pub dictionary: Full<'a>,
    #[borrowme(owned = BTreeMap<Inflection, OwnedFull>, with = self::inflections)]
    pub inflections: BTreeMap<Inflection, Fragments<'a>>,
}

impl<'a> Inflections<'a> {
    pub fn new(dictionary: Full<'a>) -> Self {
        Self {
            dictionary,
            inflections: BTreeMap::new(),
        }
    }

    /// Insert a value into this collection of inflections.
    pub(crate) fn insert(&mut self, inflect: &[Form], inflect2: &[Form], word: Fragments<'a>) {
        let mut form = crate::macro_support::fixed_map::Set::new();

        for f in inflect {
            form.insert(*f);
        }

        for f in inflect2 {
            form.insert(*f);
        }

        self.inflections.insert(crate::Inflection::new(form), word);
    }

    /// Test if any polite inflections exist.
    pub fn has_polite(&self) -> bool {
        for c in self.inflections.keys() {
            if c.form.contains(Form::Honorific) {
                return true;
            }
        }

        false
    }

    /// Test if an inflection exists.
    pub fn contains(&self, inflection: Inflection) -> bool {
        self.inflections.contains_key(&inflection)
    }

    /// Get a inflection.
    pub fn get(&self, inflection: Inflection) -> Option<&Fragments<'a>> {
        self.inflections.get(&inflection)
    }

    /// Iterate over all inflections.
    pub fn iter(&self) -> impl Iterator<Item = (&Inflection, &Fragments<'a>)> + '_ {
        self.inflections.iter()
    }
}

impl OwnedInflections {
    /// Test if an inflection exists.
    pub fn contains(&self, inflection: Inflection) -> bool {
        self.inflections.contains_key(&inflection)
    }

    /// Get a inflection.
    pub fn get(&self, inflection: Inflection) -> Option<&OwnedFull> {
        self.inflections.get(&inflection)
    }
}

mod inflections {
    use std::collections::BTreeMap;

    use crate::kana::{Fragments, OwnedFull};
    use crate::Inflection;

    pub(crate) fn to_owned(
        this: &BTreeMap<Inflection, Fragments<'_>>,
    ) -> BTreeMap<Inflection, OwnedFull> {
        let mut out = BTreeMap::new();

        for (key, value) in this {
            out.insert(
                *key,
                OwnedFull {
                    text: value.text().to_string(),
                    reading: value.reading().to_string(),
                    suffix: value.suffix().to_string(),
                },
            );
        }

        out
    }

    pub(crate) fn borrow(
        this: &BTreeMap<Inflection, OwnedFull>,
    ) -> BTreeMap<Inflection, Fragments<'_>> {
        let mut out = BTreeMap::new();

        for (key, value) in this {
            out.insert(
                *key,
                Fragments::new(
                    [value.text.as_str()],
                    [value.reading.as_str()],
                    [value.suffix.as_str()],
                ),
            );
        }

        out
    }
}
