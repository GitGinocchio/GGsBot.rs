use std::collections::HashMap;

use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "id")] Indonesian,
    #[serde(rename = "da")] Danish,
    #[serde(rename = "de")] German,
    #[serde(rename = "en-GB")] EnglishGB,
    #[serde(rename = "en-US")] EnglishUS,
    #[serde(rename = "es-ES")] Spanish,
    #[serde(rename = "es-419")] SpanishLATAM,
    #[serde(rename = "fr")] French,
    #[serde(rename = "hr")] Croatian,
    #[serde(rename = "it")] Italian,
    #[serde(rename = "lt")] Lithuanian,
    #[serde(rename = "hu")] Hungarian,
    #[serde(rename = "nl")] Dutch,
    #[serde(rename = "no")] Norwegian,
    #[serde(rename = "pl")] Polish,
    #[serde(rename = "pt-BR")] PortugueseBR,
    #[serde(rename = "ro")] Romanian,
    #[serde(rename = "fi")] Finnish,
    #[serde(rename = "sv-SE")] Swedish,
    #[serde(rename = "vi")] Vietnamese,
    #[serde(rename = "tr")] Turkish,
    #[serde(rename = "cs")] Czech,
    #[serde(rename = "el")] Greek,
    #[serde(rename = "bg")] Bulgarian,
    #[serde(rename = "ru")] Russian,
    #[serde(rename = "uk")] Ukrainian,
    #[serde(rename = "hi")] Hindi,
    #[serde(rename = "th")] Thai,
    #[serde(rename = "zh-CN")] ChineseChina,
    #[serde(rename = "ja")] Japanese,
    #[serde(rename = "zh-TW")] ChineseTaiwan,
    #[serde(rename = "ko")] Korean,
}

#[derive(Clone, Debug)]
pub enum Localization {
    Default(String),
    Map(HashMap<Locale, String>),
}

impl From<&str> for Localization {
    fn from(s: &str) -> Self {
        Self::Default(s.to_string())
    }
}

impl Localization {
    /// Estrae il nome di default (la prima stringa o la chiave "en-US" se presente)
    pub fn get_default(&self) -> String {
        match self {
            Self::Default(s) => s.clone(),
            Self::Map(m) => {
                // Discord richiede un nome di default. Cerchiamo EnglishUS o il primo disponibile.
                m.get(&Locale::EnglishUS)
                    .cloned()
                    .unwrap_or_else(|| m.values().next().cloned().unwrap_or_default())
            }
        }
    }

    pub fn get_map(&self) -> Option<&HashMap<Locale, String>> {
        match self {
            Self::Default(_) => None,
            Self::Map(m) => Some(m),
        }
    }
}