use std::collections::HashMap;
use csv::{self, StringRecord};
use std::error::Error;
use rand::Rng;
use unidecode::unidecode;

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

    fn insert(&mut self, mut word: Word) {
        let key = word.word.to_lowercase();
        let unicode = &unidecode(&word.word).to_lowercase();
        let is_unicode = word.word.eq(unicode);
        let exists = self.map.get(&key);

        if exists.is_none() && !is_unicode {
            word.references = vec![unicode.to_string()];
        }

        if exists.is_some() {
            word.references = exists.unwrap().references.clone();
        }

        self.map.insert(key, word.clone());

        if !is_unicode {
            self.insert_unicode(word);
        }
    }

    fn insert_unicode(&mut self, word: Word) {
        let unicode = &unidecode(&word.word);
        let word_unicode = self.map.get(unicode);

        let mut word_save: Word;
        if word_unicode.is_some() {
            word_save = word_unicode.unwrap().clone();
            let mut unicode_mix = word_save.references.clone();
            let conains_unicode = unicode_mix.iter().any(|reference| reference.eq(&word.word.clone()));
            if !conains_unicode {
                unicode_mix.push(word.word.clone());
            }
            word_save.references = unicode_mix;
        } else {
            word_save = word.clone();
            word_save.word = unicode.to_string();
            word_save.visible = false;
            word_save.references = vec![word.word.clone()];
        }

        self.map.insert(unicode.to_lowercase(), word_save);
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

    fn find_visible(&self, code: &String) -> Option<&Word> {
        let word = self.map.get(code);
        if word.is_some() && word.unwrap().visible {
            return word;
        }
        return Option::None;
    }

    fn find_permute_includes(&mut self, code: String, lax: Option<bool>, size: &mut Option<i64>, includes: Option<i8>) -> Vec<Word> {
        let mut code_vector: Vec<Word> = vec![];
        let includes_vector = self.find_includes(&code, includes, lax, size.clone());
        for word in includes_vector {
            code_vector.push(word.clone());
            if size.is_some() {
                let _ = size.insert(size.unwrap() - 1);
            }
        }
        return code_vector;
    }

    fn find_permute_basic(&mut self, code: String, lax: Option<bool>, size: &mut Option<i64>) -> Vec<Word> {
        if lax.is_some() {
            let words = self.find_lax(&code);
            if size.is_some() {
                let _ = size.insert(size.unwrap() - words.len() as i64);
            }
            return words.iter().cloned().cloned().collect();
        } 
        
        let mut code_vector: Vec<Word> = vec![];
        let word = self.find_visible(&code);
        if word.is_some() {
            code_vector.push(word.unwrap().clone());
            if size.is_some() {
                let _ = size.insert(size.unwrap() - 1);
            }
        }
        return code_vector;
    }

    fn find_permute_unrecognized(&mut self, code: String, size: &mut Option<i64>) -> Vec<Word> {
        let mut code_vector: Vec<Word> = vec![];
        let mut new_word = Word::empty();
        new_word.word = code;
        new_word.category = "rust-dictionary-permutation".to_string();
        code_vector.push(new_word);

        if size.is_some() {
            let _ = size.insert(size.unwrap() - 1);
        }
        return code_vector;
    }

    fn calculate_includes_condition(&self, code: &String, key: &String, word: &Word, lax: Option<bool>, position: Option<i8>) -> bool{      
        let mut action = 0;
        if position.is_some() {
            action = position.unwrap();
        }

        let mut lax_status = false;
        if lax.is_some() && lax.unwrap() {
            lax_status = self.calculate_includes_lax_condition(code, key, word, position);
        }
         
        return lax_status || match action {
            -1 => key.starts_with(code) ,
             1 => key.ends_with(code)   ,
             _ => key.contains(code)    ,
        };
    }

    fn calculate_includes_lax_condition(&self, code: &String, key: &String, word: &Word, position: Option<i8>) -> bool{
        let code_as_unicode = unidecode(&code);
        let mut action = 0;
        if position.is_some() {
            action = position.unwrap();
        }

        let mut reference_status = false;
        for reference in word.references.clone() {
            reference_status = match action {
                -1 => reference.starts_with(code) || reference.starts_with(&code_as_unicode) ,
                 1 => reference.ends_with(code)   || reference.ends_with(&code_as_unicode)   ,
                 _ => reference.contains(code)    || reference.contains(&code_as_unicode)    ,
            };
            if reference_status {
                break;
            }
        }

        let key_status = match action {
            -1 => key.starts_with(&code_as_unicode) ,
             1 => key.ends_with(&code_as_unicode)   ,
             _ => key.contains(&code_as_unicode)    ,
        };

        return reference_status || key_status;
    }

}

impl WordCollection for WordCollectionMemory {

    fn find(&mut self, code: &String) -> Option<&Word> {
        print!("{}", self.scraper(code));
        return self.find_visible(code);
    }

    fn find_lax(&self, code: &String) -> Vec<&Word> {
        let mut filter: Vec<&Word> = Vec::new();
        let word = self.map.get(&unidecode(code));
        if word.is_some() {
            if word.unwrap().visible {
                filter.push(&word.unwrap());
            }
            for reference in word.unwrap().references.clone() {
                let word_unicode = self.map.get(&reference.to_lowercase());
                if word_unicode.is_some() && word_unicode.unwrap().visible  {
                    filter.push(&word_unicode.unwrap());
                }
            }
        }
        return filter;
    }

    fn find_includes(&self, code: &String, position: Option<i8>, lax: Option<bool>, size: Option<i64>) -> Vec<&Word> {
        let mut filter: Vec<&Word> = Vec::new();
        for key in self.map.keys().clone() {
            let word = self.find_visible(key);
            if word.is_none() {
                continue;
            }
            let in_filter = filter.iter().any(|&i| i.word.eq(&word.unwrap().word));
            if !in_filter {
                let coincidence = self.calculate_includes_condition(code, key, word.unwrap(), lax, position);
                if coincidence {
                    filter.push(word.unwrap());
                    if size.is_some() && (filter.len() as i64) >= size.unwrap() {
                        return filter;
                    }
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
            let word = self.find_visible(key);
            if word.is_some() {
                word_vector.push(word.unwrap());
            }
        }
        return word_vector;
    }

    fn find_permute(&mut self, combo: &String, min: Option<i8>, exists: Option<bool>, lax: Option<bool>, includes: Option<i8>, size: Option<i64>) -> Vec<Word> {
        let permute = ComboPermuter::new(combo.to_string(), min);
        let mut word_vector: Vec<Word> = vec![];
        let size_copy = &mut size.clone();
        for code in permute.permute() {
            let mut code_vector: Vec<Word> = vec![];
            if includes.is_some() {
                let mut result = self.find_permute_includes(code.clone(), lax, size_copy, includes);
                code_vector.append(&mut result);
            } else {
                let mut result = self.find_permute_basic(code.clone(), lax, size_copy);
                code_vector.append(&mut result);
            }

            if exists.is_some() && !exists.unwrap() && code_vector.is_empty() {
                let mut result = self.find_permute_unrecognized(code.clone(), size_copy);
                code_vector.append(&mut result);
            }

            word_vector.append(&mut code_vector);

            if size_copy.is_some() && size_copy.unwrap() <= 0 {
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
            let mut collection: Vec<&Word> = self.map.values().collect();
            collection.sort_by(|a, b| a.word.to_lowercase().cmp(&b.word.to_lowercase()));
            for word in collection {
                if word.visible {
                    let _  = writer.write_record(word.as_vector());
                }
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