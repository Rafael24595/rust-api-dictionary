use serde::Deserialize;
use serde::Serialize;

use crate::configuration::dto_word::DTOWord;

const SEPARATOR: &str = "#";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
//TODO: Rewrite.
pub struct Word {
    pub word: String,
    pub unicode: String,
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

impl Word {
    
    pub fn empty() -> Word {
        return Word {
            word: String::new(),
            unicode: String::new(),
            category: String::new(),
            genre: String::new(),
            number: String::new(),
            root: String::new(),
            affix: String::new(),
            tonic: String::new(),
            syllables: String::new(),
            locale: String::new(),
            origin: String::new(),
            synonyms: String::new(),
            meaning: Vec::new()
        };
    }

    pub fn from_dto(dto: DTOWord) -> Word {
        let word = if dto.word.is_some() {dto.word.unwrap()} else {String::new()};
        let unicode = if dto.unicode.is_some() {dto.unicode.unwrap()} else {String::new()};
        let category = if dto.category.is_some() {dto.category.unwrap()} else {String::new()};
        let genre = if dto.genre.is_some() {dto.genre.unwrap()} else {String::new()};
        let number = if dto.number.is_some() {dto.number.unwrap()} else {String::new()};
        let root = if dto.root.is_some() {dto.root.unwrap()} else {String::new()};
        let affix = if dto.affix.is_some() {dto.affix.unwrap()} else {String::new()};
        let tonic = if dto.tonic.is_some() {dto.tonic.unwrap()} else {String::new()};
        let syllables = if dto.syllables.is_some() {dto.syllables.unwrap()} else {String::new()};
        let locale = if dto.locale.is_some() {dto.locale.unwrap()} else {String::new()};
        let origin = if dto.origin.is_some() {dto.origin.unwrap()} else {String::new()};
        let synonyms = if dto.synonyms.is_some() {dto.synonyms.unwrap()} else {String::new()};
        let mut meaning: Vec<String> = Vec::new();

        if dto.meaning.is_some() {
            for part in dto.meaning.unwrap().split("#") {
                meaning.push(part.to_string());
            }
        }

        return Word {
            word, unicode, category, genre, number, root, affix, tonic, syllables, locale, origin, synonyms, meaning
        };
    }

    pub fn as_vector(&self) -> Vec<std::string::String> {
        let v = self.clone();
        return vec![
            v.word,
            v.unicode,
            v.category,
            v.genre,
            v.number,
            v.root,
            v.affix,
            v.tonic,
            v.syllables,
            v.locale,
            v.origin,
            v.synonyms,
            v.meaning.join(SEPARATOR)]
    }

    pub fn as_dto(&self) -> DTOWord {
        let v = self.clone();
        return DTOWord{
            word: Option::Some(v.word),
            unicode: Option::Some(v.unicode),
            category: Option::Some(v.category),
            genre: Option::Some(v.genre),
            number: Option::Some(v.number),
            root: Option::Some(v.root),
            affix: Option::Some(v.affix),
            tonic: Option::Some(v.tonic),
            syllables: Option::Some(v.syllables),
            locale: Option::Some(v.locale),
            origin: Option::Some(v.origin),
            synonyms: Option::Some(v.synonyms),
            meaning: Option::Some(v.meaning.join(SEPARATOR))
        }
    }

}