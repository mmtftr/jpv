// string to char array mapping.
#[rustfmt::skip]
macro_rules! chars {
    ("ちょう", $o:ident) => { $o!(['ち', 'ょ', 'う']) };
    ("チョウ", $o:ident) => { $o!(['チ', 'ョ', 'ウ']) };
    ("tyô", $o:ident) => { $o!(['t', 'y', 'ô']) };
    ("chou", $o:ident) => { $o!(['c', 'h', 'o', 'u']) };
    ("りょう", $o:ident) => { $o!(['り', 'ょ', 'う']) };
    ("リョウ", $o:ident) => { $o!(['リ', 'ョ', 'ウ']) };
    ("ryô", $o:ident) => { $o!(['r', 'y', 'ô']) };
    ("ryou", $o:ident) => { $o!(['r', 'y', 'o', 'u']) };

    ("っか", $o:ident) => { $o!(['っ', 'か']) };
    ("ッカ", $o:ident) => { $o!(['ッ', 'カ']) };
    ("kka", $o:ident) => { $o!(['k', 'k', 'a']) };
    ("っが", $o:ident) => { $o!(['っ', 'が']) };
    ("ッガ", $o:ident) => { $o!(['ッ', 'ガ']) };
    ("gga", $o:ident) => { $o!(['g', 'g', 'a']) };
    ("っく", $o:ident) => { $o!(['っ', 'く']) };
    ("ック", $o:ident) => { $o!(['ッ', 'ク']) };
    ("kku", $o:ident) => { $o!(['k', 'k', 'u']) };
    ("っぐ", $o:ident) => { $o!(['っ', 'ぐ']) };
    ("ッグ", $o:ident) => { $o!(['ッ', 'グ']) };
    ("ggu", $o:ident) => { $o!(['g', 'g', 'u']) };
    ("っき", $o:ident) => { $o!(['っ', 'き']) };
    ("ッキ", $o:ident) => { $o!(['ッ', 'キ']) };
    ("kki", $o:ident) => { $o!(['k', 'k', 'i']) };
    ("っぎ", $o:ident) => { $o!(['っ', 'ぎ']) };
    ("ッギ", $o:ident) => { $o!(['ッ', 'ギ']) };
    ("ggi", $o:ident) => { $o!(['g', 'g', 'i']) };
    ("っけ", $o:ident) => { $o!(['っ', 'け']) };
    ("ッケ", $o:ident) => { $o!(['ッ', 'ケ']) };
    ("kke", $o:ident) => { $o!(['k', 'k', 'e']) };
    ("っげ", $o:ident) => { $o!(['っ', 'げ']) };
    ("ッゲ", $o:ident) => { $o!(['ッ', 'ゲ']) };
    ("gge", $o:ident) => { $o!(['g', 'g', 'e']) };
    ("っこ", $o:ident) => { $o!(['っ', 'こ']) };
    ("ッコ", $o:ident) => { $o!(['ッ', 'コ']) };
    ("kko", $o:ident) => { $o!(['k', 'k', 'o']) };
    ("っご", $o:ident) => { $o!(['っ', 'ご']) };
    ("ッゴ", $o:ident) => { $o!(['ッ', 'ゴ']) };
    ("ggo", $o:ident) => { $o!(['g', 'g', 'o']) };

    ("っさ", $o:ident) => { $o!(['っ', 'さ']) };
    ("ッサ", $o:ident) => { $o!(['ッ', 'サ']) };
    ("ssa", $o:ident) => { $o!(['s', 's', 'a']) };
    ("っざ", $o:ident) => { $o!(['っ', 'ざ']) };
    ("ッザ", $o:ident) => { $o!(['ッ', 'ザ']) };
    ("zza", $o:ident) => { $o!(['z', 'z', 'a']) };
    ("っし", $o:ident) => { $o!(['っ', 'し']) };
    ("ッシ", $o:ident) => { $o!(['ッ', 'シ']) };
    ("ssi", $o:ident) => { $o!(['s', 's', 'i']) };
    ("sshi", $o:ident) => { $o!(['s', 's', 'h', 'i']) };
    ("っじ", $o:ident) => { $o!(['っ', 'じ']) };
    ("ッジ", $o:ident) => { $o!(['ッ', 'ジ']) };
    ("jji", $o:ident) => { $o!(['j', 'j', 'i']) };
    ("っす", $o:ident) => { $o!(['っ', 'す']) };
    ("ッス", $o:ident) => { $o!(['ッ', 'ス']) };
    ("ssu", $o:ident) => { $o!(['s', 's', 'u']) };
    ("っず", $o:ident) => { $o!(['っ', 'ず']) };
    ("ッズ", $o:ident) => { $o!(['ッ', 'ズ']) };
    ("zzu", $o:ident) => { $o!(['z', 'z', 'u']) };
    ("っせ", $o:ident) => { $o!(['っ', 'せ']) };
    ("ッセ", $o:ident) => { $o!(['ッ', 'セ']) };
    ("sse", $o:ident) => { $o!(['s', 's', 'e']) };
    ("っぜ", $o:ident) => { $o!(['っ', 'ぜ']) };
    ("ッゼ", $o:ident) => { $o!(['ッ', 'ゼ']) };
    ("zze", $o:ident) => { $o!(['z', 'z', 'e']) };
    ("っそ", $o:ident) => { $o!(['っ', 'そ']) };
    ("ッソ", $o:ident) => { $o!(['ッ', 'ソ']) };
    ("sso", $o:ident) => { $o!(['s', 's', 'o']) };
    ("っぞ", $o:ident) => { $o!(['っ', 'ぞ']) };
    ("ッゾ", $o:ident) => { $o!(['ッ', 'ゾ']) };
    ("zzo", $o:ident) => { $o!(['z', 'z', 'o']) };

    ("った", $o:ident) => { $o!(['っ', 'た']) };
    ("ッタ", $o:ident) => { $o!(['ッ', 'タ']) };
    ("tta", $o:ident) => { $o!(['t', 't', 'a']) };
    ("っだ", $o:ident) => { $o!(['っ', 'だ']) };
    ("ッダ", $o:ident) => { $o!(['ッ', 'ダ']) };
    ("dda", $o:ident) => { $o!(['d', 'd', 'a']) };
    ("っち", $o:ident) => { $o!(['っ', 'ち']) };
    ("ッチ", $o:ident) => { $o!(['ッ', 'チ']) };
    ("tti", $o:ident) => { $o!(['t', 't', 'i']) };
    ("っぢ", $o:ident) => { $o!(['っ', 'ぢ']) };
    ("ッヂ", $o:ident) => { $o!(['ッ', 'ヂ']) };
    ("ddi", $o:ident) => { $o!(['d', 'd', 'i']) };
    ("っつ", $o:ident) => { $o!(['っ', 'つ']) };
    ("ッツ", $o:ident) => { $o!(['ッ', 'ツ']) };
    ("ttu", $o:ident) => { $o!(['t', 't', 'u']) };
    ("ttsu", $o:ident) => { $o!(['t', 't', 's', 'u']) };
    ("っづ", $o:ident) => { $o!(['っ', 'づ']) };
    ("ッヅ", $o:ident) => { $o!(['ッ', 'ヅ']) };
    ("ddu", $o:ident) => { $o!(['d', 'd', 'u']) };
    ("って", $o:ident) => { $o!(['っ', 'て']) };
    ("ッテ", $o:ident) => { $o!(['ッ', 'テ']) };
    ("tte", $o:ident) => { $o!(['t', 't', 'e']) };
    ("っで", $o:ident) => { $o!(['っ', 'で']) };
    ("ッデ", $o:ident) => { $o!(['ッ', 'デ']) };
    ("dde", $o:ident) => { $o!(['d', 'd', 'e']) };
    ("っと", $o:ident) => { $o!(['っ', 'と']) };
    ("ット", $o:ident) => { $o!(['ッ', 'ト']) };
    ("tto", $o:ident) => { $o!(['t', 't', 'o']) };
    ("っど", $o:ident) => { $o!(['っ', 'ど']) };
    ("ッド", $o:ident) => { $o!(['ッ', 'ド']) };
    ("ddo", $o:ident) => { $o!(['d', 'd', 'o']) };
    
    ("っは", $o:ident) => { $o!(['っ', 'は']) };
    ("ッハ", $o:ident) => { $o!(['ッ', 'ハ']) };
    ("hha", $o:ident) => { $o!(['h', 'h', 'a']) };
    ("っば", $o:ident) => { $o!(['っ', 'ば']) };
    ("ッバ", $o:ident) => { $o!(['ッ', 'バ']) };
    ("bba", $o:ident) => { $o!(['b', 'b', 'a']) };
    ("っぱ", $o:ident) => { $o!(['っ', 'ぱ']) };
    ("ッパ", $o:ident) => { $o!(['ッ', 'パ']) };
    ("ppa", $o:ident) => { $o!(['p', 'p', 'a']) };
    ("っひ", $o:ident) => { $o!(['っ', 'ひ']) };
    ("ッヒ", $o:ident) => { $o!(['ッ', 'ヒ']) };
    ("hhi", $o:ident) => { $o!(['h', 'h', 'i']) };
    ("っび", $o:ident) => { $o!(['っ', 'び']) };
    ("ッビ", $o:ident) => { $o!(['ッ', 'ビ']) };
    ("bbi", $o:ident) => { $o!(['b', 'b', 'i']) };
    ("っぴ", $o:ident) => { $o!(['っ', 'ぴ']) };
    ("ッピ", $o:ident) => { $o!(['ッ', 'ピ']) };
    ("ppi", $o:ident) => { $o!(['p', 'p', 'i']) };
    ("っふ", $o:ident) => { $o!(['っ', 'ふ']) };
    ("ッフ", $o:ident) => { $o!(['ッ', 'フ']) };
    ("ffu", $o:ident) => { $o!(['f', 'f', 'u']) };
    ("っぶ", $o:ident) => { $o!(['っ', 'ぶ']) };
    ("ッブ", $o:ident) => { $o!(['ッ', 'ブ']) };
    ("bbu", $o:ident) => { $o!(['b', 'b', 'u']) };
    ("っぷ", $o:ident) => { $o!(['っ', 'ぷ']) };
    ("ップ", $o:ident) => { $o!(['ッ', 'プ']) };
    ("ppu", $o:ident) => { $o!(['p', 'p', 'u']) };
    ("っへ", $o:ident) => { $o!(['っ', 'へ']) };
    ("ッヘ", $o:ident) => { $o!(['ッ', 'ヘ']) };
    ("hhe", $o:ident) => { $o!(['h', 'h', 'e']) };
    ("っべ", $o:ident) => { $o!(['っ', 'べ']) };
    ("ッベ", $o:ident) => { $o!(['ッ', 'ベ']) };
    ("bbe", $o:ident) => { $o!(['b', 'b', 'e']) };
    ("っぺ", $o:ident) => { $o!(['っ', 'ぺ']) };
    ("ッペ", $o:ident) => { $o!(['ッ', 'ペ']) };
    ("ppe", $o:ident) => { $o!(['p', 'p', 'e']) };
    ("っほ", $o:ident) => { $o!(['っ', 'ほ']) };
    ("ッホ", $o:ident) => { $o!(['ッ', 'ホ']) };
    ("hho", $o:ident) => { $o!(['h', 'h', 'o']) };
    ("っぼ", $o:ident) => { $o!(['っ', 'ぼ']) };
    ("ッボ", $o:ident) => { $o!(['ッ', 'ボ']) };
    ("bbo", $o:ident) => { $o!(['b', 'b', 'o']) };

    ("っら", $o:ident) => { $o!(['っ', 'ら']) };
    ("ッラ", $o:ident) => { $o!(['ッ', 'ラ']) };
    ("rra", $o:ident) => { $o!(['r', 'r', 'a']) };
    ("っり", $o:ident) => { $o!(['っ', 'り']) };
    ("ッリ", $o:ident) => { $o!(['ッ', 'リ']) };
    ("rri", $o:ident) => { $o!(['r', 'r', 'i']) };
    ("っる", $o:ident) => { $o!(['っ', 'る']) };
    ("ッル", $o:ident) => { $o!(['ッ', 'ル']) };
    ("rru", $o:ident) => { $o!(['r', 'r', 'u']) };
    ("っれ", $o:ident) => { $o!(['っ', 'れ']) };
    ("ッレ", $o:ident) => { $o!(['ッ', 'レ']) };
    ("rre", $o:ident) => { $o!(['r', 'r', 'e']) };
    ("っろ", $o:ident) => { $o!(['っ', 'ろ']) };
    ("ッロ", $o:ident) => { $o!(['ッ', 'ロ']) };
    ("rro", $o:ident) => { $o!(['r', 'r', 'o']) };

    ("った", $o:ident) => { $o!(['っ', 'た']) };
    ("ッタ", $o:ident) => { $o!(['ッ', 'タ']) };
    ("tta", $o:ident) => { $o!(['t', 't', 'a']) };
    ("うう", $o:ident) => { $o!(['う', 'う']) };
    ("ウウ", $o:ident) => { $o!(['ウ', 'ウ']) };
    ("û", $o:ident) => { $o!(['û']) };
    ("uu", $o:ident) => { $o!(['u', 'u']) };
    ("おう", $o:ident) => { $o!(['お', 'う']) };
    ("オウ", $o:ident) => { $o!(['オ', 'ウ']) };
    ("ô", $o:ident) => { $o!(['ô']) };
    ("ou", $o:ident) => { $o!(['o', 'u']) };
    ("ぎゃ", $o:ident) => { $o!(['ぎ', 'ゃ']) };
    ("ギャ", $o:ident) => { $o!(['ギ', 'ャ']) };
    ("gya", $o:ident) => { $o!(['g', 'y', 'a']) };
    ("きゃ", $o:ident) => { $o!(['き', 'ゃ']) };
    ("キャ", $o:ident) => { $o!(['キ', 'ャ']) };
    ("kya", $o:ident) => { $o!(['k', 'y', 'a']) };
    ("ぎゅ", $o:ident) => { $o!(['ぎ', 'ゅ']) };
    ("ギュ", $o:ident) => { $o!(['ギ', 'ュ']) };
    ("gyu", $o:ident) => { $o!(['g', 'y', 'u']) };
    ("きゅ", $o:ident) => { $o!(['き', 'ゅ']) };
    ("キュ", $o:ident) => { $o!(['キ', 'ュ']) };
    ("kyu", $o:ident) => { $o!(['k', 'y', 'u']) };
    ("ぎょ", $o:ident) => { $o!(['ぎ', 'ょ']) };
    ("ギョ", $o:ident) => { $o!(['ギ', 'ョ']) };
    ("gyo", $o:ident) => { $o!(['g', 'y', 'o']) };
    ("きょ", $o:ident) => { $o!(['き', 'ょ']) };
    ("キョ", $o:ident) => { $o!(['キ', 'ョ']) };
    ("kyo", $o:ident) => { $o!(['k', 'y', 'o']) };
    ("しゃ", $o:ident) => { $o!(['し', 'ゃ']) };
    ("シャ", $o:ident) => { $o!(['シ', 'ャ']) };
    ("sha", $o:ident) => { $o!(['s', 'h', 'a']) };
    ("sya", $o:ident) => { $o!(['s', 'y', 'a']) };
    ("じゃ", $o:ident) => { $o!(['じ', 'ゃ']) };
    ("ジャ", $o:ident) => { $o!(['ジ', 'ャ']) };
    ("ja", $o:ident) => { $o!(['j', 'a']) };
    ("jya", $o:ident) => { $o!(['j', 'y', 'a']) };
    ("zya", $o:ident) => { $o!(['z', 'y', 'a']) };
    ("しゅ", $o:ident) => { $o!(['し', 'ゅ']) };
    ("シュ", $o:ident) => { $o!(['シ', 'ュ']) };
    ("shu", $o:ident) => { $o!(['s', 'h', 'u']) };
    ("syu", $o:ident) => { $o!(['s', 'y', 'u']) };
    ("じゅ", $o:ident) => { $o!(['じ', 'ゅ']) };
    ("ジュ", $o:ident) => { $o!(['ジ', 'ュ']) };
    ("ju", $o:ident) => { $o!(['j', 'u']) };
    ("jyu", $o:ident) => { $o!(['j', 'y', 'u']) };
    ("zyu", $o:ident) => { $o!(['z', 'y', 'u']) };
    ("しょ", $o:ident) => { $o!(['し', 'ょ']) };
    ("ショ", $o:ident) => { $o!(['シ', 'ョ']) };
    ("sho", $o:ident) => { $o!(['s', 'h', 'o']) };
    ("syo", $o:ident) => { $o!(['s', 'y', 'o']) };
    ("じょ", $o:ident) => { $o!(['じ', 'ょ']) };
    ("ジョ", $o:ident) => { $o!(['ジ', 'ョ']) };
    ("jo", $o:ident) => { $o!(['j', 'o']) };
    ("jyo", $o:ident) => { $o!(['j', 'y', 'o']) };
    ("zyo", $o:ident) => { $o!(['z', 'y', 'o']) };
    ("ぢゃ", $o:ident) => { $o!(['ぢ', 'ゃ']) };
    ("ヂャ", $o:ident) => { $o!(['ヂ', 'ャ']) };
    ("dha", $o:ident) => { $o!(['d', 'h', 'a']) };
    ("dya", $o:ident) => { $o!(['d', 'y', 'a']) };
    ("ちゃ", $o:ident) => { $o!(['ち', 'ゃ']) };
    ("チャ", $o:ident) => { $o!(['チ', 'ャ']) };
    ("cha", $o:ident) => { $o!(['c', 'h', 'a']) };
    ("cya", $o:ident) => { $o!(['c', 'y', 'a']) };
    ("tya", $o:ident) => { $o!(['t', 'y', 'a']) };
    ("ぢゅ", $o:ident) => { $o!(['ぢ', 'ゅ']) };
    ("ヂュ", $o:ident) => { $o!(['ヂ', 'ュ']) };
    ("dhu", $o:ident) => { $o!(['d', 'h', 'u']) };
    ("dyu", $o:ident) => { $o!(['d', 'y', 'u']) };
    ("ちゅ", $o:ident) => { $o!(['ち', 'ゅ']) };
    ("チュ", $o:ident) => { $o!(['チ', 'ュ']) };
    ("chu", $o:ident) => { $o!(['c', 'h', 'u']) };
    ("cyu", $o:ident) => { $o!(['c', 'y', 'u']) };
    ("tyu", $o:ident) => { $o!(['t', 'y', 'u']) };
    ("ぢょ", $o:ident) => { $o!(['ぢ', 'ょ']) };
    ("ヂョ", $o:ident) => { $o!(['ヂ', 'ョ']) };
    ("dho", $o:ident) => { $o!(['d', 'h', 'o']) };
    ("dyo", $o:ident) => { $o!(['d', 'y', 'o']) };
    ("ちょ", $o:ident) => { $o!(['ち', 'ょ']) };
    ("チョ", $o:ident) => { $o!(['チ', 'ョ']) };
    ("cho", $o:ident) => { $o!(['c', 'h', 'o']) };
    ("cyo", $o:ident) => { $o!(['c', 'y', 'o']) };
    ("tyo", $o:ident) => { $o!(['t', 'y', 'o']) };
    ("にゃ", $o:ident) => { $o!(['に', 'ゃ']) };
    ("ニャ", $o:ident) => { $o!(['ニ', 'ャ']) };
    ("nya", $o:ident) => { $o!(['n', 'y', 'a']) };
    ("にゅ", $o:ident) => { $o!(['に', 'ゅ']) };
    ("ニュ", $o:ident) => { $o!(['ニ', 'ュ']) };
    ("nyu", $o:ident) => { $o!(['n', 'y', 'u']) };
    ("にょ", $o:ident) => { $o!(['に', 'ょ']) };
    ("ニョ", $o:ident) => { $o!(['ニ', 'ョ']) };
    ("nyo", $o:ident) => { $o!(['n', 'y', 'o']) };
    ("びゃ", $o:ident) => { $o!(['び', 'ゃ']) };
    ("ビャ", $o:ident) => { $o!(['ビ', 'ャ']) };
    ("bya", $o:ident) => { $o!(['b', 'y', 'a']) };
    ("ひゃ", $o:ident) => { $o!(['ひ', 'ゃ']) };
    ("ヒャ", $o:ident) => { $o!(['ヒ', 'ャ']) };
    ("hya", $o:ident) => { $o!(['h', 'y', 'a']) };
    ("ぴゃ", $o:ident) => { $o!(['ぴ', 'ゃ']) };
    ("ピャ", $o:ident) => { $o!(['ピ', 'ャ']) };
    ("pya", $o:ident) => { $o!(['p', 'y', 'a']) };
    ("びゅ", $o:ident) => { $o!(['び', 'ゅ']) };
    ("ビュ", $o:ident) => { $o!(['ビ', 'ュ']) };
    ("byu", $o:ident) => { $o!(['b', 'y', 'u']) };
    ("ひゅ", $o:ident) => { $o!(['ひ', 'ゅ']) };
    ("ヒュ", $o:ident) => { $o!(['ヒ', 'ュ']) };
    ("hyu", $o:ident) => { $o!(['h', 'y', 'u']) };
    ("ぴゅ", $o:ident) => { $o!(['ぴ', 'ゅ']) };
    ("ピュ", $o:ident) => { $o!(['ピ', 'ュ']) };
    ("pyu", $o:ident) => { $o!(['p', 'y', 'u']) };
    ("びょ", $o:ident) => { $o!(['び', 'ょ']) };
    ("ビョ", $o:ident) => { $o!(['ビ', 'ョ']) };
    ("byo", $o:ident) => { $o!(['b', 'y', 'o']) };
    ("ひょ", $o:ident) => { $o!(['ひ', 'ょ']) };
    ("ヒョ", $o:ident) => { $o!(['ヒ', 'ョ']) };
    ("hyo", $o:ident) => { $o!(['h', 'y', 'o']) };
    ("ぴょ", $o:ident) => { $o!(['ぴ', 'ょ']) };
    ("ピョ", $o:ident) => { $o!(['ピ', 'ョ']) };
    ("pyo", $o:ident) => { $o!(['p', 'y', 'o']) };
    ("みゃ", $o:ident) => { $o!(['み', 'ゃ']) };
    ("ミャ", $o:ident) => { $o!(['ミ', 'ャ']) };
    ("mya", $o:ident) => { $o!(['m', 'y', 'a']) };
    ("みゅ", $o:ident) => { $o!(['み', 'ゅ']) };
    ("ミュ", $o:ident) => { $o!(['ミ', 'ュ']) };
    ("myu", $o:ident) => { $o!(['m', 'y', 'u']) };
    ("みょ", $o:ident) => { $o!(['み', 'ょ']) };
    ("ミョ", $o:ident) => { $o!(['ミ', 'ョ']) };
    ("myo", $o:ident) => { $o!(['m', 'y', 'o']) };
    ("りゃ", $o:ident) => { $o!(['り', 'ゃ']) };
    ("リャ", $o:ident) => { $o!(['リ', 'ャ']) };
    ("rya", $o:ident) => { $o!(['r', 'y', 'a']) };
    ("りゅ", $o:ident) => { $o!(['り', 'ゅ']) };
    ("リュ", $o:ident) => { $o!(['リ', 'ュ']) };
    ("ryu", $o:ident) => { $o!(['r', 'y', 'u']) };
    ("りょ", $o:ident) => { $o!(['り', 'ょ']) };
    ("リョ", $o:ident) => { $o!(['リ', 'ョ']) };
    ("ryo", $o:ident) => { $o!(['r', 'y', 'o']) };
    ("いぇ", $o:ident) => { $o!(['い', 'ぇ']) };
    ("イェ", $o:ident) => { $o!(['イ', 'ェ']) };
    ("ye", $o:ident) => { $o!(['y', 'e']) };
    ("ゔぁ", $o:ident) => { $o!(['ゔ', 'ぁ']) };
    ("ヴァ", $o:ident) => { $o!(['ヴ', 'ァ']) };
    ("va", $o:ident) => { $o!(['v', 'a']) };
    ("ゔぃ", $o:ident) => { $o!(['ゔ', 'ぃ']) };
    ("ヴィ", $o:ident) => { $o!(['ヴ', 'ィ']) };
    ("vi", $o:ident) => { $o!(['v', 'i']) };
    ("うぃ", $o:ident) => { $o!(['う', 'ぃ']) };
    ("ウィ", $o:ident) => { $o!(['ウ', 'ィ']) };
    ("whi", $o:ident) => { $o!(['w', 'h', 'i']) };
    ("ゔぇ", $o:ident) => { $o!(['ゔ', 'ぇ']) };
    ("ヴェ", $o:ident) => { $o!(['ヴ', 'ェ']) };
    ("ve", $o:ident) => { $o!(['v', 'e']) };
    ("うぇ", $o:ident) => { $o!(['う', 'ぇ']) };
    ("ウェ", $o:ident) => { $o!(['ウ', 'ェ']) };
    ("whe", $o:ident) => { $o!(['w', 'h', 'e']) };
    ("ゔぉ", $o:ident) => { $o!(['ゔ', 'ぉ']) };
    ("ヴォ", $o:ident) => { $o!(['ヴ', 'ォ']) };
    ("vo", $o:ident) => { $o!(['v', 'o']) };
    ("うぉ", $o:ident) => { $o!(['う', 'ぉ']) };
    ("ウォ", $o:ident) => { $o!(['ウ', 'ォ']) };
    ("who", $o:ident) => { $o!(['w', 'h', 'o']) };
    ("ゔゅ", $o:ident) => { $o!(['ゔ', 'ゅ']) };
    ("ヴュ", $o:ident) => { $o!(['ヴ', 'ュ']) };
    ("vyu", $o:ident) => { $o!(['v', 'y', 'u']) };
    ("きぃ", $o:ident) => { $o!(['き', 'ぃ']) };
    ("キィ", $o:ident) => { $o!(['キ', 'ィ']) };
    ("kyi", $o:ident) => { $o!(['k', 'y', 'i']) };
    ("きぇ", $o:ident) => { $o!(['き', 'ぇ']) };
    ("キェ", $o:ident) => { $o!(['キ', 'ェ']) };
    ("kye", $o:ident) => { $o!(['k', 'y', 'e']) };
    ("ぐぁ", $o:ident) => { $o!(['ぐ', 'ぁ']) };
    ("グァ", $o:ident) => { $o!(['グ', 'ァ']) };
    ("gwa", $o:ident) => { $o!(['g', 'w', 'a']) };
    ("くぁ", $o:ident) => { $o!(['く', 'ぁ']) };
    ("クァ", $o:ident) => { $o!(['ク', 'ァ']) };
    ("kwa", $o:ident) => { $o!(['k', 'w', 'a']) };
    ("qa", $o:ident) => { $o!(['q', 'a']) };
    ("くぃ", $o:ident) => { $o!(['く', 'ぃ']) };
    ("クィ", $o:ident) => { $o!(['ク', 'ィ']) };
    ("kwi", $o:ident) => { $o!(['k', 'w', 'i']) };
    ("qi", $o:ident) => { $o!(['q', 'i']) };
    ("くぅ", $o:ident) => { $o!(['く', 'ぅ']) };
    ("クゥ", $o:ident) => { $o!(['ク', 'ゥ']) };
    ("kwu", $o:ident) => { $o!(['k', 'w', 'u']) };
    ("くぇ", $o:ident) => { $o!(['く', 'ぇ']) };
    ("クェ", $o:ident) => { $o!(['ク', 'ェ']) };
    ("kwe", $o:ident) => { $o!(['k', 'w', 'e']) };
    ("qe", $o:ident) => { $o!(['q', 'e']) };
    ("くぉ", $o:ident) => { $o!(['く', 'ぉ']) };
    ("クォ", $o:ident) => { $o!(['ク', 'ォ']) };
    ("kwo", $o:ident) => { $o!(['k', 'w', 'o']) };
    ("qo", $o:ident) => { $o!(['q', 'o']) };
    ("じぃ", $o:ident) => { $o!(['じ', 'ぃ']) };
    ("ジィ", $o:ident) => { $o!(['ジ', 'ィ']) };
    ("jyi", $o:ident) => { $o!(['j', 'y', 'i']) };
    ("じぇ", $o:ident) => { $o!(['じ', 'ぇ']) };
    ("ジェ", $o:ident) => { $o!(['ジ', 'ェ']) };
    ("je", $o:ident) => { $o!(['j', 'e']) };
    ("jye", $o:ident) => { $o!(['j', 'y', 'e']) };
    ("zye", $o:ident) => { $o!(['z', 'y', 'e']) };
    ("しぇ", $o:ident) => { $o!(['し', 'ぇ']) };
    ("シェ", $o:ident) => { $o!(['シ', 'ェ']) };
    ("she", $o:ident) => { $o!(['s', 'h', 'e']) };
    ("sye", $o:ident) => { $o!(['s', 'y', 'e']) };
    ("ぢぃ", $o:ident) => { $o!(['ぢ', 'ぃ']) };
    ("ヂィ", $o:ident) => { $o!(['ヂ', 'ィ']) };
    ("dyi", $o:ident) => { $o!(['d', 'y', 'i']) };
    ("ちぇ", $o:ident) => { $o!(['ち', 'ぇ']) };
    ("チェ", $o:ident) => { $o!(['チ', 'ェ']) };
    ("che", $o:ident) => { $o!(['c', 'h', 'e']) };
    ("tye", $o:ident) => { $o!(['t', 'y', 'e']) };
    ("ぢぇ", $o:ident) => { $o!(['ぢ', 'ぇ']) };
    ("ヂェ", $o:ident) => { $o!(['ヂ', 'ェ']) };
    ("dhe", $o:ident) => { $o!(['d', 'h', 'e']) };
    ("dye", $o:ident) => { $o!(['d', 'y', 'e']) };
    ("つぁ", $o:ident) => { $o!(['つ', 'ぁ']) };
    ("ツァ", $o:ident) => { $o!(['ツ', 'ァ']) };
    ("tsa", $o:ident) => { $o!(['t', 's', 'a']) };
    ("つぃ", $o:ident) => { $o!(['つ', 'ぃ']) };
    ("ツィ", $o:ident) => { $o!(['ツ', 'ィ']) };
    ("tsi", $o:ident) => { $o!(['t', 's', 'i']) };
    ("つぇ", $o:ident) => { $o!(['つ', 'ぇ']) };
    ("ツェ", $o:ident) => { $o!(['ツ', 'ェ']) };
    ("tse", $o:ident) => { $o!(['t', 's', 'e']) };
    ("つぉ", $o:ident) => { $o!(['つ', 'ぉ']) };
    ("ツォ", $o:ident) => { $o!(['ツ', 'ォ']) };
    ("tso", $o:ident) => { $o!(['t', 's', 'o']) };
    ("でぃ", $o:ident) => { $o!(['で', 'ぃ']) };
    ("ディ", $o:ident) => { $o!(['デ', 'ィ']) };
    ("d'i", $o:ident) => { $o!(['d', '\'', 'i']) };
    ("てぃ", $o:ident) => { $o!(['て', 'ぃ']) };
    ("ティ", $o:ident) => { $o!(['テ', 'ィ']) };
    ("t'i", $o:ident) => { $o!(['t', '\'', 'i']) };
    ("thi", $o:ident) => { $o!(['t', 'h', 'i']) };
    ("でゅ", $o:ident) => { $o!(['で', 'ゅ']) };
    ("デュ", $o:ident) => { $o!(['デ', 'ュ']) };
    ("d'yu", $o:ident) => { $o!(['d', '\'', 'y', 'u']) };
    ("てゅ", $o:ident) => { $o!(['て', 'ゅ']) };
    ("テュ", $o:ident) => { $o!(['テ', 'ュ']) };
    ("t'yu", $o:ident) => { $o!(['t', '\'', 'y', 'u']) };
    ("thu", $o:ident) => { $o!(['t', 'h', 'u']) };
    ("どぅ", $o:ident) => { $o!(['ど', 'ぅ']) };
    ("ドゥ", $o:ident) => { $o!(['ド', 'ゥ']) };
    ("d'u", $o:ident) => { $o!(['d', '\'', 'u']) };
    ("dwu", $o:ident) => { $o!(['d', 'w', 'u']) };
    ("とぅ", $o:ident) => { $o!(['と', 'ぅ']) };
    ("トゥ", $o:ident) => { $o!(['ト', 'ゥ']) };
    ("t'u", $o:ident) => { $o!(['t', '\'', 'u']) };
    ("twu", $o:ident) => { $o!(['t', 'w', 'u']) };
    ("にぃ", $o:ident) => { $o!(['に', 'ぃ']) };
    ("ニィ", $o:ident) => { $o!(['ニ', 'ィ']) };
    ("nyi", $o:ident) => { $o!(['n', 'y', 'i']) };
    ("にぇ", $o:ident) => { $o!(['に', 'ぇ']) };
    ("ニェ", $o:ident) => { $o!(['ニ', 'ェ']) };
    ("nye", $o:ident) => { $o!(['n', 'y', 'e']) };
    ("ひぃ", $o:ident) => { $o!(['ひ', 'ぃ']) };
    ("ヒィ", $o:ident) => { $o!(['ヒ', 'ィ']) };
    ("hyi", $o:ident) => { $o!(['h', 'y', 'i']) };
    ("ぴぃ", $o:ident) => { $o!(['ぴ', 'ぃ']) };
    ("ピィ", $o:ident) => { $o!(['ピ', 'ィ']) };
    ("pyi", $o:ident) => { $o!(['p', 'y', 'i']) };
    ("びぇ", $o:ident) => { $o!(['び', 'ぇ']) };
    ("ビェ", $o:ident) => { $o!(['ビ', 'ェ']) };
    ("bye", $o:ident) => { $o!(['b', 'y', 'e']) };
    ("ひぇ", $o:ident) => { $o!(['ひ', 'ぇ']) };
    ("ヒェ", $o:ident) => { $o!(['ヒ', 'ェ']) };
    ("hye", $o:ident) => { $o!(['h', 'y', 'e']) };
    ("ぴぇ", $o:ident) => { $o!(['ぴ', 'ぇ']) };
    ("ピェ", $o:ident) => { $o!(['ピ', 'ェ']) };
    ("pye", $o:ident) => { $o!(['p', 'y', 'e']) };
    ("ふぁ", $o:ident) => { $o!(['ふ', 'ぁ']) };
    ("ファ", $o:ident) => { $o!(['フ', 'ァ']) };
    ("fa", $o:ident) => { $o!(['f', 'a']) };
    ("hwa", $o:ident) => { $o!(['h', 'w', 'a']) };
    ("ふぃ", $o:ident) => { $o!(['ふ', 'ぃ']) };
    ("フィ", $o:ident) => { $o!(['フ', 'ィ']) };
    ("fi", $o:ident) => { $o!(['f', 'i']) };
    ("fyi", $o:ident) => { $o!(['f', 'y', 'i']) };
    ("hwi", $o:ident) => { $o!(['h', 'w', 'i']) };
    ("ふぇ", $o:ident) => { $o!(['ふ', 'ぇ']) };
    ("フェ", $o:ident) => { $o!(['フ', 'ェ']) };
    ("fe", $o:ident) => { $o!(['f', 'e']) };
    ("fye", $o:ident) => { $o!(['f', 'y', 'e']) };
    ("hwe", $o:ident) => { $o!(['h', 'w', 'e']) };
    ("ふぉ", $o:ident) => { $o!(['ふ', 'ぉ']) };
    ("フォ", $o:ident) => { $o!(['フ', 'ォ']) };
    ("fo", $o:ident) => { $o!(['f', 'o']) };
    ("hwo", $o:ident) => { $o!(['h', 'w', 'o']) };
    ("ふゃ", $o:ident) => { $o!(['ふ', 'ゃ']) };
    ("フャ", $o:ident) => { $o!(['フ', 'ャ']) };
    ("fya", $o:ident) => { $o!(['f', 'y', 'a']) };
    ("ふゅ", $o:ident) => { $o!(['ふ', 'ゅ']) };
    ("フュ", $o:ident) => { $o!(['フ', 'ュ']) };
    ("fyu", $o:ident) => { $o!(['f', 'y', 'u']) };
    ("hwyu", $o:ident) => { $o!(['h', 'w', 'y', 'u']) };
    ("ふょ", $o:ident) => { $o!(['ふ', 'ょ']) };
    ("フョ", $o:ident) => { $o!(['フ', 'ョ']) };
    ("fyo", $o:ident) => { $o!(['f', 'y', 'o']) };
    ("みぃ", $o:ident) => { $o!(['み', 'ぃ']) };
    ("ミィ", $o:ident) => { $o!(['ミ', 'ィ']) };
    ("myi", $o:ident) => { $o!(['m', 'y', 'i']) };
    ("みぇ", $o:ident) => { $o!(['み', 'ぇ']) };
    ("ミェ", $o:ident) => { $o!(['ミ', 'ェ']) };
    ("mye", $o:ident) => { $o!(['m', 'y', 'e']) };
    ("りぃ", $o:ident) => { $o!(['り', 'ぃ']) };
    ("リィ", $o:ident) => { $o!(['リ', 'ィ']) };
    ("ryi", $o:ident) => { $o!(['r', 'y', 'i']) };
    ("りぇ", $o:ident) => { $o!(['り', 'ぇ']) };
    ("リェ", $o:ident) => { $o!(['リ', 'ェ']) };
    ("rye", $o:ident) => { $o!(['r', 'y', 'e']) };
    ("あ", $o:ident) => { $o!(['あ']) };
    ("ア", $o:ident) => { $o!(['ア']) };
    ("a", $o:ident) => { $o!(['a']) };
    ("い", $o:ident) => { $o!(['い']) };
    ("イ", $o:ident) => { $o!(['イ']) };
    ("i", $o:ident) => { $o!(['i']) };
    ("yi", $o:ident) => { $o!(['y', 'i']) };
    ("う", $o:ident) => { $o!(['う']) };
    ("ウ", $o:ident) => { $o!(['ウ']) };
    ("u", $o:ident) => { $o!(['u']) };
    ("え", $o:ident) => { $o!(['え']) };
    ("エ", $o:ident) => { $o!(['エ']) };
    ("e", $o:ident) => { $o!(['e']) };
    ("お", $o:ident) => { $o!(['お']) };
    ("オ", $o:ident) => { $o!(['オ']) };
    ("o", $o:ident) => { $o!(['o']) };
    ("が", $o:ident) => { $o!(['が']) };
    ("ガ", $o:ident) => { $o!(['ガ']) };
    ("ga", $o:ident) => { $o!(['g', 'a']) };
    ("か", $o:ident) => { $o!(['か']) };
    ("カ", $o:ident) => { $o!(['カ']) };
    ("ka", $o:ident) => { $o!(['k', 'a']) };
    ("ぎ", $o:ident) => { $o!(['ぎ']) };
    ("ギ", $o:ident) => { $o!(['ギ']) };
    ("gi", $o:ident) => { $o!(['g', 'i']) };
    ("き", $o:ident) => { $o!(['き']) };
    ("キ", $o:ident) => { $o!(['キ']) };
    ("ki", $o:ident) => { $o!(['k', 'i']) };
    ("ぐ", $o:ident) => { $o!(['ぐ']) };
    ("グ", $o:ident) => { $o!(['グ']) };
    ("gu", $o:ident) => { $o!(['g', 'u']) };
    ("く", $o:ident) => { $o!(['く']) };
    ("ク", $o:ident) => { $o!(['ク']) };
    ("ku", $o:ident) => { $o!(['k', 'u']) };
    ("げ", $o:ident) => { $o!(['げ']) };
    ("ゲ", $o:ident) => { $o!(['ゲ']) };
    ("ge", $o:ident) => { $o!(['g', 'e']) };
    ("け", $o:ident) => { $o!(['け']) };
    ("ケ", $o:ident) => { $o!(['ケ']) };
    ("ke", $o:ident) => { $o!(['k', 'e']) };
    ("ご", $o:ident) => { $o!(['ご']) };
    ("ゴ", $o:ident) => { $o!(['ゴ']) };
    ("go", $o:ident) => { $o!(['g', 'o']) };
    ("こ", $o:ident) => { $o!(['こ']) };
    ("コ", $o:ident) => { $o!(['コ']) };
    ("ko", $o:ident) => { $o!(['k', 'o']) };
    ("さ", $o:ident) => { $o!(['さ']) };
    ("サ", $o:ident) => { $o!(['サ']) };
    ("sa", $o:ident) => { $o!(['s', 'a']) };
    ("ざ", $o:ident) => { $o!(['ざ']) };
    ("ザ", $o:ident) => { $o!(['ザ']) };
    ("za", $o:ident) => { $o!(['z', 'a']) };
    ("し", $o:ident) => { $o!(['し']) };
    ("シ", $o:ident) => { $o!(['シ']) };
    ("si", $o:ident) => { $o!(['s', 'i']) };
    ("shi", $o:ident) => { $o!(['s', 'h', 'i']) };
    ("じ", $o:ident) => { $o!(['じ']) };
    ("ジ", $o:ident) => { $o!(['ジ']) };
    ("ji", $o:ident) => { $o!(['j', 'i']) };
    ("zi", $o:ident) => { $o!(['z', 'i']) };
    ("す", $o:ident) => { $o!(['す']) };
    ("ス", $o:ident) => { $o!(['ス']) };
    ("su", $o:ident) => { $o!(['s', 'u']) };
    ("ず", $o:ident) => { $o!(['ず']) };
    ("ズ", $o:ident) => { $o!(['ズ']) };
    ("zu", $o:ident) => { $o!(['z', 'u']) };
    ("せ", $o:ident) => { $o!(['せ']) };
    ("セ", $o:ident) => { $o!(['セ']) };
    ("se", $o:ident) => { $o!(['s', 'e']) };
    ("ぜ", $o:ident) => { $o!(['ぜ']) };
    ("ゼ", $o:ident) => { $o!(['ゼ']) };
    ("ze", $o:ident) => { $o!(['z', 'e']) };
    ("そ", $o:ident) => { $o!(['そ']) };
    ("ソ", $o:ident) => { $o!(['ソ']) };
    ("so", $o:ident) => { $o!(['s', 'o']) };
    ("ぞ", $o:ident) => { $o!(['ぞ']) };
    ("ゾ", $o:ident) => { $o!(['ゾ']) };
    ("zo", $o:ident) => { $o!(['z', 'o']) };
    ("だ", $o:ident) => { $o!(['だ']) };
    ("ダ", $o:ident) => { $o!(['ダ']) };
    ("da", $o:ident) => { $o!(['d', 'a']) };
    ("た", $o:ident) => { $o!(['た']) };
    ("タ", $o:ident) => { $o!(['タ']) };
    ("ta", $o:ident) => { $o!(['t', 'a']) };
    ("ぢ", $o:ident) => { $o!(['ぢ']) };
    ("ヂ", $o:ident) => { $o!(['ヂ']) };
    ("dhi", $o:ident) => { $o!(['d', 'h', 'i']) };
    ("di", $o:ident) => { $o!(['d', 'i']) };
    ("ち", $o:ident) => { $o!(['ち']) };
    ("チ", $o:ident) => { $o!(['チ']) };
    ("chi", $o:ident) => { $o!(['c', 'h', 'i']) };
    ("ti", $o:ident) => { $o!(['t', 'i']) };
    ("づ", $o:ident) => { $o!(['づ']) };
    ("ヅ", $o:ident) => { $o!(['ヅ']) };
    ("du", $o:ident) => { $o!(['d', 'u']) };
    ("dzu", $o:ident) => { $o!(['d', 'z', 'u']) };
    ("つ", $o:ident) => { $o!(['つ']) };
    ("ツ", $o:ident) => { $o!(['ツ']) };
    ("tsu", $o:ident) => { $o!(['t', 's', 'u']) };
    ("tu", $o:ident) => { $o!(['t', 'u']) };
    ("で", $o:ident) => { $o!(['で']) };
    ("デ", $o:ident) => { $o!(['デ']) };
    ("de", $o:ident) => { $o!(['d', 'e']) };
    ("て", $o:ident) => { $o!(['て']) };
    ("テ", $o:ident) => { $o!(['テ']) };
    ("te", $o:ident) => { $o!(['t', 'e']) };
    ("ど", $o:ident) => { $o!(['ど']) };
    ("ド", $o:ident) => { $o!(['ド']) };
    ("do", $o:ident) => { $o!(['d', 'o']) };
    ("と", $o:ident) => { $o!(['と']) };
    ("ト", $o:ident) => { $o!(['ト']) };
    ("to", $o:ident) => { $o!(['t', 'o']) };
    ("な", $o:ident) => { $o!(['な']) };
    ("ナ", $o:ident) => { $o!(['ナ']) };
    ("na", $o:ident) => { $o!(['n', 'a']) };
    ("に", $o:ident) => { $o!(['に']) };
    ("ニ", $o:ident) => { $o!(['ニ']) };
    ("ni", $o:ident) => { $o!(['n', 'i']) };
    ("ぬ", $o:ident) => { $o!(['ぬ']) };
    ("ヌ", $o:ident) => { $o!(['ヌ']) };
    ("nu", $o:ident) => { $o!(['n', 'u']) };
    ("ね", $o:ident) => { $o!(['ね']) };
    ("ネ", $o:ident) => { $o!(['ネ']) };
    ("ne", $o:ident) => { $o!(['n', 'e']) };
    ("の", $o:ident) => { $o!(['の']) };
    ("ノ", $o:ident) => { $o!(['ノ']) };
    ("no", $o:ident) => { $o!(['n', 'o']) };
    ("ば", $o:ident) => { $o!(['ば']) };
    ("バ", $o:ident) => { $o!(['バ']) };
    ("ba", $o:ident) => { $o!(['b', 'a']) };
    ("は", $o:ident) => { $o!(['は']) };
    ("ハ", $o:ident) => { $o!(['ハ']) };
    ("ha", $o:ident) => { $o!(['h', 'a']) };
    ("ぱ", $o:ident) => { $o!(['ぱ']) };
    ("パ", $o:ident) => { $o!(['パ']) };
    ("pa", $o:ident) => { $o!(['p', 'a']) };
    ("び", $o:ident) => { $o!(['び']) };
    ("ビ", $o:ident) => { $o!(['ビ']) };
    ("bi", $o:ident) => { $o!(['b', 'i']) };
    ("ひ", $o:ident) => { $o!(['ひ']) };
    ("ヒ", $o:ident) => { $o!(['ヒ']) };
    ("hi", $o:ident) => { $o!(['h', 'i']) };
    ("ぴ", $o:ident) => { $o!(['ぴ']) };
    ("ピ", $o:ident) => { $o!(['ピ']) };
    ("pi", $o:ident) => { $o!(['p', 'i']) };
    ("ぶ", $o:ident) => { $o!(['ぶ']) };
    ("ブ", $o:ident) => { $o!(['ブ']) };
    ("bu", $o:ident) => { $o!(['b', 'u']) };
    ("ふ", $o:ident) => { $o!(['ふ']) };
    ("フ", $o:ident) => { $o!(['フ']) };
    ("hu", $o:ident) => { $o!(['h', 'u']) };
    ("ぷ", $o:ident) => { $o!(['ぷ']) };
    ("プ", $o:ident) => { $o!(['プ']) };
    ("pu", $o:ident) => { $o!(['p', 'u']) };
    ("べ", $o:ident) => { $o!(['べ']) };
    ("ベ", $o:ident) => { $o!(['ベ']) };
    ("be", $o:ident) => { $o!(['b', 'e']) };
    ("へ", $o:ident) => { $o!(['へ']) };
    ("ヘ", $o:ident) => { $o!(['ヘ']) };
    ("he", $o:ident) => { $o!(['h', 'e']) };
    ("ぺ", $o:ident) => { $o!(['ぺ']) };
    ("ペ", $o:ident) => { $o!(['ペ']) };
    ("pe", $o:ident) => { $o!(['p', 'e']) };
    ("ぼ", $o:ident) => { $o!(['ぼ']) };
    ("ボ", $o:ident) => { $o!(['ボ']) };
    ("bo", $o:ident) => { $o!(['b', 'o']) };
    ("ほ", $o:ident) => { $o!(['ほ']) };
    ("ホ", $o:ident) => { $o!(['ホ']) };
    ("ho", $o:ident) => { $o!(['h', 'o']) };
    ("ぽ", $o:ident) => { $o!(['ぽ']) };
    ("ポ", $o:ident) => { $o!(['ポ']) };
    ("po", $o:ident) => { $o!(['p', 'o']) };
    ("ま", $o:ident) => { $o!(['ま']) };
    ("マ", $o:ident) => { $o!(['マ']) };
    ("ma", $o:ident) => { $o!(['m', 'a']) };
    ("み", $o:ident) => { $o!(['み']) };
    ("ミ", $o:ident) => { $o!(['ミ']) };
    ("mi", $o:ident) => { $o!(['m', 'i']) };
    ("む", $o:ident) => { $o!(['む']) };
    ("ム", $o:ident) => { $o!(['ム']) };
    ("mu", $o:ident) => { $o!(['m', 'u']) };
    ("め", $o:ident) => { $o!(['め']) };
    ("メ", $o:ident) => { $o!(['メ']) };
    ("me", $o:ident) => { $o!(['m', 'e']) };
    ("も", $o:ident) => { $o!(['も']) };
    ("モ", $o:ident) => { $o!(['モ']) };
    ("mo", $o:ident) => { $o!(['m', 'o']) };
    ("や", $o:ident) => { $o!(['や']) };
    ("ヤ", $o:ident) => { $o!(['ヤ']) };
    ("ya", $o:ident) => { $o!(['y', 'a']) };
    ("ゆ", $o:ident) => { $o!(['ゆ']) };
    ("ユ", $o:ident) => { $o!(['ユ']) };
    ("yu", $o:ident) => { $o!(['y', 'u']) };
    ("よ", $o:ident) => { $o!(['よ']) };
    ("ヨ", $o:ident) => { $o!(['ヨ']) };
    ("yo", $o:ident) => { $o!(['y', 'o']) };
    ("ら", $o:ident) => { $o!(['ら']) };
    ("ラ", $o:ident) => { $o!(['ラ']) };
    ("ra", $o:ident) => { $o!(['r', 'a']) };
    ("り", $o:ident) => { $o!(['り']) };
    ("リ", $o:ident) => { $o!(['リ']) };
    ("ri", $o:ident) => { $o!(['r', 'i']) };
    ("る", $o:ident) => { $o!(['る']) };
    ("ル", $o:ident) => { $o!(['ル']) };
    ("ru", $o:ident) => { $o!(['r', 'u']) };
    ("れ", $o:ident) => { $o!(['れ']) };
    ("レ", $o:ident) => { $o!(['レ']) };
    ("re", $o:ident) => { $o!(['r', 'e']) };
    ("ろ", $o:ident) => { $o!(['ろ']) };
    ("ロ", $o:ident) => { $o!(['ロ']) };
    ("ro", $o:ident) => { $o!(['r', 'o']) };
    ("わ", $o:ident) => { $o!(['わ']) };
    ("ワ", $o:ident) => { $o!(['ワ']) };
    ("wa", $o:ident) => { $o!(['w', 'a']) };
    ("ゐ", $o:ident) => { $o!(['ゐ']) };
    ("ヰ", $o:ident) => { $o!(['ヰ']) };
    ("wi", $o:ident) => { $o!(['w', 'i']) };
    ("wyi", $o:ident) => { $o!(['w', 'y', 'i']) };
    ("を", $o:ident) => { $o!(['を']) };
    ("ヲ", $o:ident) => { $o!(['ヲ']) };
    ("wo", $o:ident) => { $o!(['w', 'o']) };
    ("ん", $o:ident) => { $o!(['ん']) };
    ("ン", $o:ident) => { $o!(['ン']) };
    ("n'", $o:ident) => { $o!(['n', '\'']) };
    ("nn", $o:ident) => { $o!(['n', 'n']) };
    ("ゑ", $o:ident) => { $o!(['ゑ']) };
    ("ヱ", $o:ident) => { $o!(['ヱ']) };
    ("we", $o:ident) => { $o!(['w', 'e']) };
    ("wye", $o:ident) => { $o!(['w', 'y', 'e']) };
    ("ぁ", $o:ident) => { $o!(['ぁ']) };
    ("ァ", $o:ident) => { $o!(['ァ']) };
    ("la", $o:ident) => { $o!(['l', 'a']) };
    ("xa", $o:ident) => { $o!(['x', 'a']) };
    ("ぃ", $o:ident) => { $o!(['ぃ']) };
    ("ィ", $o:ident) => { $o!(['ィ']) };
    ("li", $o:ident) => { $o!(['l', 'i']) };
    ("lyi", $o:ident) => { $o!(['l', 'y', 'i']) };
    ("xi", $o:ident) => { $o!(['x', 'i']) };
    ("ぅ", $o:ident) => { $o!(['ぅ']) };
    ("ゥ", $o:ident) => { $o!(['ゥ']) };
    ("lu", $o:ident) => { $o!(['l', 'u']) };
    ("xu", $o:ident) => { $o!(['x', 'u']) };
    ("ぇ", $o:ident) => { $o!(['ぇ']) };
    ("ェ", $o:ident) => { $o!(['ェ']) };
    ("le", $o:ident) => { $o!(['l', 'e']) };
    ("lye", $o:ident) => { $o!(['l', 'y', 'e']) };
    ("xe", $o:ident) => { $o!(['x', 'e']) };
    ("ぉ", $o:ident) => { $o!(['ぉ']) };
    ("ォ", $o:ident) => { $o!(['ォ']) };
    ("lo", $o:ident) => { $o!(['l', 'o']) };
    ("xo", $o:ident) => { $o!(['x', 'o']) };
    ("っ", $o:ident) => { $o!(['っ']) };
    ("ッ", $o:ident) => { $o!(['ッ']) };
    ("xtu", $o:ident) => { $o!(['x', 't', 'u']) };
    ("ゃ", $o:ident) => { $o!(['ゃ']) };
    ("ャ", $o:ident) => { $o!(['ャ']) };
    ("lya", $o:ident) => { $o!(['l', 'y', 'a']) };
    ("xya", $o:ident) => { $o!(['x', 'y', 'a']) };
    ("ゅ", $o:ident) => { $o!(['ゅ']) };
    ("ュ", $o:ident) => { $o!(['ュ']) };
    ("lyu", $o:ident) => { $o!(['l', 'y', 'u']) };
    ("xyu", $o:ident) => { $o!(['x', 'y', 'u']) };
    ("ょ", $o:ident) => { $o!(['ょ']) };
    ("ョ", $o:ident) => { $o!(['ョ']) };
    ("lyo", $o:ident) => { $o!(['l', 'y', 'o']) };
    ("xyo", $o:ident) => { $o!(['x', 'y', 'o']) };
    ("ゎ", $o:ident) => { $o!(['ゎ']) };
    ("ヮ", $o:ident) => { $o!(['ヮ']) };
    ("xwa", $o:ident) => { $o!(['x', 'w', 'a']) };
    ("ゔ", $o:ident) => { $o!(['ゔ']) };
    ("ヴ", $o:ident) => { $o!(['ヴ']) };
    ("vu", $o:ident) => { $o!(['v', 'u']) };
    ("xka", $o:ident) => { $o!(['x', 'k', 'a']) };
    ("xke", $o:ident) => { $o!(['x', 'k', 'e']) };
    ("ヵ", $o:ident) => { $o!(['ヵ']) };
    ("ヶ", $o:ident) => { $o!(['ヶ']) };
    ("ー", $o:ident) => { $o!(['ー']) };
    ("-", $o:ident) => { $o!(['-']) };
    ("-", $o:ident) => { $o!(['-']) };
    ("^", $o:ident) => { $o!(['^']) };
}
