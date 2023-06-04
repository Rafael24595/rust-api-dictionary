use std::collections::HashMap;
use csv::{self, StringRecord};
use std::error::Error;
use rand::Rng;
//use futures::executor;

use crate::configuration::word_collection::WordCollection;
use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;
use crate::configuration::dto_word::DTOWord;
//use crate::configuration::diccionary::rae_raider::RaeRaider;

#[allow(dead_code)]
pub struct WordCollectionMemory {
    headers: StringRecord,
    pub map: HashMap<String, Word>
}

const SOURCE_PATH: &str = "./assets/Dictionary_es.csv";

unsafe impl Send for WordCollectionMemory{}
unsafe impl Sync for WordCollectionMemory{}

impl WordCollectionMemory {

    pub fn new() -> impl WordCollection {
        return WordCollectionMemory {
            headers: StringRecord::new(),
            map: HashMap::new()
        }
    }

    fn insert(&mut self, word: Word) {
        let key = word.word.clone().to_lowercase();
        self.map.insert(key, word);
    }

    fn find_random_positions(&self, keys: Vec<String>, size: Option<i64>) ->  Vec<usize>{
        let mut finish = false;
        let map_len = self.map.len();
        let mut position_vector: Vec<usize> = vec![];
        while !finish {
            let mut rng = rand::thread_rng();
            let position = rng.gen_range(0..keys.len());
            if !position_vector.contains(&position){
                position_vector.push(position)
            }
            if size.is_none() || map_len == position_vector.len() || (position_vector.len() as i64) == size.unwrap() {
                finish = true;
            }
        }
    
        return position_vector;
    }

    /*async fn test(&mut self, code: &String) -> bool{
        let mut rae = RaeRaider::new(code.to_string());
        let mut word = self.map.get(code).unwrap().clone();
        rae.load().await;
        let descs = rae.loot_descriptions().unwrap();
        word.meaning = Some(descs.join("#"));;
        self.map.insert(code.to_string(), word);
        return true;
    }*/

}

impl WordCollection for WordCollectionMemory {

    fn find(&mut self, code: &String) -> Option<&Word> {
        //let test = self.test(code);
        //executor::block_on(test);
        return self.map.get(code);
    }

    fn find_includes(&self, code: &String, size: Option<i64>) -> Vec<&Word> {
        let keys = self.map.keys();
        let mut filter: Vec<&Word> = Vec::new();
        for key in keys.clone() {
            let word = self.map.get(key);
            if key.contains(code) && word.is_some() {
                filter.push(word.unwrap());
                if filter.len() == keys.len() || (size.is_some() && (filter.len() as i64) >= size.unwrap()) {
                    return filter;
                }
            }
        }
        return filter;
    }

    fn find_random(&self, size: Option<i64>) ->  Vec<&Word> {
        let keys = self.map.keys().cloned().collect::<Vec<String>>();
        let mut position_vector: Vec<usize> = self.find_random_positions(keys.clone(), size);

        let mut word_vector: Vec<&Word> = vec![];
        for position in position_vector.iter_mut() {
            let key = keys.get(position.clone()).unwrap();
            let word = self.map.get(key);
            if word.is_some() {
                word_vector.push(word.unwrap());
            }
        }

        return word_vector;
    }

}

impl Dependency for WordCollectionMemory {

    fn on_exit(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn on_init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(SOURCE_PATH)?;
        self.headers = reader.headers()?.clone();

        for result in reader.deserialize() {
            let dto: DTOWord = result?;
            if dto.word.is_some() {
                let word = Word {
                    word: if dto.word.is_some() {dto.word.unwrap()} else {String::new()},
                    category: if dto.category.is_some() {dto.category.unwrap()} else {String::new()},
                    genre: if dto.genre.is_some() {dto.genre.unwrap()} else {String::new()},
                    number: if dto.number.is_some() {dto.number.unwrap()} else {String::new()},
                    root: if dto.root.is_some() {dto.root.unwrap()} else {String::new()},
                    affix: if dto.affix.is_some() {dto.affix.unwrap()} else {String::new()},
                    tonic: if dto.tonic.is_some() {dto.tonic.unwrap()} else {String::new()},
                    syllables: if dto.syllables.is_some() {dto.syllables.unwrap()} else {String::new()},
                    locale: if dto.locale.is_some() {dto.locale.unwrap()} else {String::new()},
                    origin: if dto.origin.is_some() {dto.origin.unwrap()} else {String::new()},
                    synonyms: if dto.synonyms.is_some() {dto.synonyms.unwrap()} else {String::new()},
                    meaning: if dto.meaning.is_some() {dto.meaning.unwrap()} else {String::new()}
                };
                self.insert(word);
            }
        }

        Ok(())
    }

}