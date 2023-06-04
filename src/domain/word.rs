use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
//TODO: Rewrite.
pub struct Word {
    pub word: String,
    pub category: String,
    pub genre: String,
    pub number: String,
    pub root: String,
    pub affix: String, 
    pub tonic: String, 
    pub syllables: String, 
    pub locale: String, 
    pub origin: String, 
    pub synonyms: String,
    pub meaning: Vec<String>
}