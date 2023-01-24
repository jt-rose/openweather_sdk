use std::fmt;

pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Azerbaijani,
    Bulgarian,
    Catalan,
    Czech,
    Danish,
    German,
    Greek,
    English,
    Basque,
    Persian,
    Finnish,
    French,
    Galician,
    Hebrew,
    Hindi,
    Croatian,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Latvian,
    Lithuanian,
    Macedonian,
    Norwegian,
    Dutch,
    Polish,
    Portuguese,
    Portugues_Brasil,
    Romanian,
    Russian,
    Swedish, // "se" also listed
    Slovenian,
    Spanish, // "es" also listed
    Serbian,
    Thai,
    Turkish,
    Ukrainian, // "ua" also listed
    Vietnamese,
    Chinese_Simplified,
    Chinese_Traditional,
    Zulu
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Afrikaans => "af",
                Self::Albanian => "al",
                Self::Arabic => "ar",
                Self::Azerbaijani => "az",
                Self::Bulgarian => "bg",
                Self::Catalan => "ca",
                Self::Czech => "cz",
                Self::Danish => "da",
                Self::German => "de",
                Self::Greek => "el",
                Self::English => "en",
                Self::Basque => "eu",
                Self::Persian => "fa",
                Self::Finnish => "fa",
                Self::French => "fa",
                Self::Galician => "gl",
                Self::Hebrew => "he",
                Self::Hindi => "hi",
                Self::Croatian => "hr",
                Self::Hungarian => "hu",
                Self::Indonesian => "id",
                Self::Italian => "it",
                Self::Japanese => "ja",
                Self::Korean => "kr",
                Self::Latvian => "la",
                Self::Lithuanian => "lt",
                Self::Macedonian => "mk",
                Self::Norwegian => "no",
                Self::Dutch => "nl",
                Self::Polish => "pl",
                Self::Portuguese => "pt",
                Self::Portugues_Brasil => "pt_br",
                Self::Romanian => "ro",
                Self::Russian => "ru",
                Self::Swedish => "sv", // "se" also listed
                Self::Slovenian => "sl",
                Self::Spanish => "sp", // "es" also listed
                Self::Serbian => "sr",
                Self::Thai => "th",
                Self::Turkish => "tr",
                Self::Ukrainian => "uk", // "ua" also listed
                Self::Vietnamese => "vi",
                Self::Chinese_Simplified => "zh_cn",
                Self::Chinese_Traditional => "zh_tw",
                Self::Zulu => "zu"
            }
        )
    }
}