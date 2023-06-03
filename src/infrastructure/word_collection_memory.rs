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

}

impl WordCollection for WordCollectionMemory {

    fn find(&self, code: &String) -> Option<&Word> {
        return self.map.get(code);
    }

    fn find_includes(&self, code: &String, size: Option<i64>) -> Vec<Option<&Word>> {
        let keys = self.map.keys();
        let mut filter: Vec<Option<&Word>> = Vec::new();
        for key in keys.clone() {
            if key.contains(code) {
                filter.push(self.find(key));
                if filter.len() == keys.len() || (size.is_some() && (filter.len() as i64) >= size.unwrap()) {
                    return filter;
                }
            }
        }
        return filter;
    }

    fn find_random(&self, size: Option<i64>) -> Vec<Option<&Word>> {
        let keys = self.map.keys().cloned().collect::<Vec<String>>();
        let mut position_vector: Vec<usize> = self.find_random_positions(keys.clone(), size);

        let mut word_vector: Vec<Option<&Word>> = vec![];
        for position in position_vector.iter_mut() {
            let key = keys.get(position.clone()).unwrap();
            let word = self.map.get(key);
            word_vector.push(word);
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
            let record: Word = result?;
            if record.word.is_some() {
                self.insert(record);
            }
        }

        Ok(())
    }

}