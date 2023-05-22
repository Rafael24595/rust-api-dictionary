#[path = "../commons/dependency.rs"] pub(crate) mod dependency;
#[path = "word_collection.rs"] pub(crate) mod word_collection;

use crate::word;
use std::collections::HashMap;
use csv::{self, StringRecord};
use std::error::Error;
use word::Word;
use dependency::Dependency;
use word_collection::WordCollection;

#[allow(dead_code)]
pub struct WordCollectionMemory {
    headers: StringRecord,
    pub map: HashMap<String, Word>
}

const SOURCE_PATH: &str = "./assets/Dictionary_es.csv";

impl WordCollectionMemory {

    pub fn new() -> impl WordCollection + Dependency {
        return WordCollectionMemory {
            headers: StringRecord::new(),
            map: HashMap::new()
        }
    }

}

impl WordCollection for WordCollectionMemory {

    fn find(&self, code: &String) -> Option<&Word> {
        return self.map.get(code);
    }

    fn find_includes(&self, code: &String) -> Vec<Option<&Word>> {
        let mut filter: Vec<Option<&Word>> = Vec::new();
        for key in self.map.keys() {
            if key.contains(code) {
                filter.push(self.find(key))
            }
        }
        return filter;
    }

    fn insert(&mut self, word: Word) {
        self.map.insert(word.word.clone().unwrap(), word);
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