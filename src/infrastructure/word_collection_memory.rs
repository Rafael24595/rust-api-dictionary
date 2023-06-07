use std::collections::HashMap;
use csv::{self, StringRecord};
use std::error::Error;
use rand::Rng;

use crate::configuration::word_collection::WordCollection;
use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;
use crate::configuration::dto_word::DTOWord;
use crate::configuration::diccionary::rae_raider::RaeRaider;
use crate::configuration::diccionary::combo_permuter::ComboPermuter;

#[allow(dead_code)]
pub struct WordCollectionMemory {
    headers: StringRecord,
    map: HashMap<String, Word>,
    enable_scraper: bool,
    enable_rebuild: bool
}

const SOURCE_PATH: &str = "./assets/dictionary_es.csv";
const SOURCE_PATH_UPDATED: &str = "./assets/dictionary_updated_es.csv";

unsafe impl Send for WordCollectionMemory{}
unsafe impl Sync for WordCollectionMemory{}

impl WordCollectionMemory {

    pub fn new(args: HashMap<String, String>) -> impl WordCollection {
        let enable_scraper = args.get("ENABLE_SCRAPER");
        let enable_rebuild = args.get("ENABLE_REBUILD");
        return WordCollectionMemory {
            headers: StringRecord::new(),
            map: HashMap::new(),
            enable_scraper: if enable_scraper.is_some() {enable_scraper.unwrap().trim().parse().unwrap()} else {false},
            enable_rebuild: if enable_rebuild.is_some() {enable_rebuild.unwrap().trim().parse().unwrap()} else {false}
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

    fn scraper(&mut self, code: &String) -> bool{
        let word = self.map.get(code);
        if self.enable_scraper && word.is_some() && word.unwrap().meaning.is_empty() {
            let mut rae = RaeRaider::new(code.to_string());
            if rae.load().is_ok() {
                let descs = rae.loot_descriptions();
                if descs.is_ok() {
                    let mut new_word = word.unwrap().clone();
                    new_word.meaning = descs.unwrap();
                    self.insert(new_word);

                    return true;
                }
            }

        }
        return false;
    }

}

impl WordCollection for WordCollectionMemory {

    fn find(&mut self, code: &String) -> Option<&Word> {
        print!("{}", self.scraper(code));
        return self.map.get(code);
    }

    fn find_includes(&self, code: &String, position: Option<i8>, size: Option<i64>) -> Vec<&Word> {
        let keys = self.map.keys();
        let mut filter: Vec<&Word> = Vec::new();
        for key in keys.clone() {
            let word = self.map.get(key);
            let coincidence = 
                if position.is_some() && position.unwrap() == -1 {key.starts_with(code)} 
                else if position.is_some() && position.unwrap() == 1 {key.ends_with(code)} 
                else {key.contains(code)};
            if coincidence && word.is_some() {
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

    fn find_permute(&mut self, combo: &String, size: Option<i64>, exists: Option<bool>) -> Vec<Word> {
        let permute = ComboPermuter::new(combo.to_string());
        let mut word_vector: Vec<Word> = vec![];
        for code in permute.permute() {
            let word = self.map.get(&code);
            if word.is_some() {
                word_vector.push(word.unwrap().clone());
            } else if exists.is_some() && !exists.unwrap() {
                let mut new_word = Word::empty();
                new_word.word = code;
                new_word.category = "rust-dictionary-permutation".to_string();
                word_vector.push(new_word);
            }
            if size.is_some() && (word_vector.len() as i64) >= size.unwrap() {
                return word_vector;
            }
        }
        return word_vector; 
    }

}

impl Dependency for WordCollectionMemory {

    fn on_exit(&mut self) -> Result<(), Box<dyn Error>> {
        if self.enable_rebuild {
            let mut writer = csv::Writer::from_path(SOURCE_PATH_UPDATED)?;
    
            let _ = writer.write_record(&self.headers);
            for value in self.map.values() {
                let _  = writer.write_record(value.as_vector());
            }
    
            writer.flush()?;
        }
        Ok(())
    }

    fn on_init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(SOURCE_PATH)?;
        self.headers = reader.headers()?.clone();

        for result in reader.deserialize() {
            let dto: DTOWord = result?;
            if dto.word.is_some() {
                let word = Word::from_dto(dto);
                self.insert(word);
            }
        }

        Ok(())
    }

}