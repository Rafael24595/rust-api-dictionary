use std::collections::HashMap;
use csv::{self, StringRecord};
use std::error::Error;
use rand::Rng;

use crate::configuration::word_collection::WordCollection;
use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;

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
        let key = word.word.clone().unwrap().to_lowercase();
        self.map.insert(key, word);
    }

    fn find_single_random(&self) -> Option<&Word> {
        let mut rng = rand::thread_rng();
        let keys = self.map.keys().cloned().collect::<Vec<String>>();
        let position = rng.gen_range(0..keys.len());
        let key = keys.get(position).unwrap();
        return self.map.get(key);
    }

}

impl WordCollection for WordCollectionMemory {

    fn find(&self, code: &String) -> Option<&Word> {
        return self.map.get(code);
    }

    fn find_includes(&self, code: &String, size: Option<i64>) -> Vec<Option<&Word>> {
        let mut filter: Vec<Option<&Word>> = Vec::new();
        for key in self.map.keys() {
            if key.contains(code) {
                filter.push(self.find(key));
                if size.is_some() && (filter.len() as i64) >= size.unwrap() {
                    return filter;
                }
            }
        }
        return filter;
    }

    fn find_random(&self, size: Option<i64>) -> Vec<Option<&Word>> {
        let mut map: HashMap<String, Option<&Word>> = HashMap::new();
        let mut finish = false;
        while !finish {
            let word = self.find_single_random();
            let key = word.unwrap().word.clone().unwrap();
            map.insert(key, word);
            if size.is_none() || self.map.len() == map.len() || (map.len() as i64) == size.unwrap() {
                finish = true;
            }
        }
        return map.values().cloned().collect::<Vec<Option<&Word>>>();
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
            let record: Word = result?;
            if record.word.is_some() {
                self.insert(record);
            }
        }

        Ok(())
    }

}