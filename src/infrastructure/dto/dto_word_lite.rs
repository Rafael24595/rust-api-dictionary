use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DTOWordLite {
    pub word: Option<String>,
    pub category: Option<String>,
    pub genre: Option<String>,
    pub number: Option<String>,
    pub root: Option<String>,
    pub affix: Option<String>, 
    pub tonic: Option<String>, 
    pub syllables: Option<String>, 
    pub locale: Option<String>, 
    pub origin: Option<String>, 
    pub synonyms: Option<String>,
    pub meaning: Option<String>
}