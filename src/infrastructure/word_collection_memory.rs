use std::collections::{HashMap, BTreeMap};
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
    map: BTreeMap<String, Word>,
    enable_scraper: bool,
    enable_rebuild: bool,
    source_file: String,
    rebuild_file: String
}

unsafe impl Send for WordCollectionMemory{}
unsafe impl Sync for WordCollectionMemory{}

impl WordCollectionMemory {

    pub fn new(args: HashMap<String, String>) -> impl WordCollection {
        let enable_scraper_string = args.get("ENABLE_SCRAPER");
        let enable_rebuild_string = args.get("ENABLE_REBUILD");
        let enable_scraper = if enable_scraper_string.is_some() {enable_scraper_string.unwrap().trim().parse().unwrap()} else {false};
        let enable_rebuild = if enable_rebuild_string.is_some() {enable_rebuild_string.unwrap().trim().parse().unwrap()} else {false};

        let source_file = args.get("SOURCE_FILE");
        if source_file.is_none() {
            panic!("Cannot initalize WordColection dependency: Source not defined.");
        }
        let rebuild_file = args.get("REBUILD_FILE");
        if rebuild_file.is_none() && enable_rebuild {
            panic!("Cannot initalize WordColection dependency: Rebuild file not defined.");
        }
        return WordCollectionMemory {
            headers: StringRecord::new(),
            map:  BTreeMap::new(),
            enable_scraper: enable_scraper,
            enable_rebuild: enable_rebuild,
            source_file: source_file.unwrap().to_string(),
            rebuild_file: rebuild_file.unwrap().to_string()
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

        let word_save: Word;
        if word_unicode.is_none() {
            word_save = self.create_unicode(word, unicode);
        } else {
            word_save = self.modify_unicode(word, word_unicode.unwrap());
        }

        self.map.insert(unicode.to_lowercase(), word_save);
    }

    fn modify_unicode(&self, word: Word, word_unicode: &Word) -> Word {
        let mut word_save = word_unicode.clone();
        let mut unicode_mix = word_save.references.clone();
        let conains_unicode = unicode_mix.iter().any(|reference| reference.eq(&word.word.clone()));
        if !conains_unicode {
            unicode_mix.push(word.word.clone());
        }
        word_save.references = unicode_mix;

        return word_save;
    }

    fn create_unicode(&self, word: Word, unicode: &String) -> Word {
        let mut word_save = word.clone();
        word_save.word = unicode.to_string();
        word_save.visible = false;
        word_save.references = vec![word.word.clone()];

        return word_save;
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

    fn find_references(&self, word: &Word) -> Vec<&Word> {
        let mut filter: Vec<&Word> = Vec::new();
        for reference in word.references.clone() {
            let word_unicode = self.map.get(&reference.to_lowercase());
            if word_unicode.is_some() && word_unicode.unwrap().visible  {
                filter.push(&word_unicode.unwrap());
            }
        }
        return filter;
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

    fn find_includes_condition(&self, code: &String, lax: Option<bool>, position: Option<i8>) -> Vec<(&String, &Word)> {      
        let mut action = 0;
        if position.is_some() {
            action = position.unwrap();
        }

        let mut word_range: Vec<(&String, &Word)> = match action {
            -1 => self.map.range(code.clone()..).take_while(|(k, _)| k.starts_with(code)).collect() ,
             1 => self.map.range(code.clone()..).take_while(|(k, _)| k.ends_with(code)).collect()   ,
             _ => self.map.range(code.clone()..).take_while(|(k, _)| k.contains(code)).collect()    ,
        };

        if lax.is_some() && lax.unwrap() {
            let mut word_lax_range: Vec<(&String, &Word)> = self.find_includes_lax_condition(code, word_range.clone(), action);
            word_range.append(&mut word_lax_range);
        }

        return word_range;
    }

    fn find_includes_lax_condition(&self, code: &String, word_range: Vec<(&String, &Word)>, action: i8) -> Vec<(&String, &Word)> {
        let code_as_unicode = &unidecode(&code);
        let word_lax_range: Vec<(&String, &Word)>  = match action {
            -1 => self.map.range(code_as_unicode.clone()..).take_while(|(k, _)| 
                !word_range.iter().any(|reference| reference.0.eq(k.as_str())) && 
                 k.starts_with(code_as_unicode)).collect() ,

             1 => self.map.range(code_as_unicode.clone()..).take_while(|(k, _)| 
                !word_range.iter().any(|reference| reference.0.eq(k.as_str())) && 
                 k.ends_with(code_as_unicode)).collect()   ,

             _ => self.map.range(code_as_unicode.clone()..).take_while(|(k, _)| 
                !word_range.iter().any(|reference| reference.0.eq(k.as_str())) && 
                 k.contains(code_as_unicode)).collect()    ,
        };

        return word_lax_range;
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
            let mut references = self.find_references(&word.unwrap());
            filter.append(&mut references);
        }
        return filter;
    }

    fn find_includes(&self, code: &String, position: Option<i8>, lax: Option<bool>, size: Option<i64>) -> Vec<&Word> {
        let word_range: Vec<(&String, &Word)> = self.find_includes_condition(code, lax, position);
        let mut filter: Vec<&Word> = Vec::new();

        for range in word_range {
            let mut words: Vec<&Word> = vec![];
            if lax.is_none() || (lax.is_some() && !lax.unwrap()) {
                let visible = self.find_visible(range.0);
                if visible.is_some() {
                    words = vec![visible.unwrap()]
                }
            } else {
                words = self.find_lax(range.0);
            }

            for word in words {
                if !filter.iter().any(|reference| reference.word.eq(&word.word)) {
                    filter.push(word);
                }

                if size.is_some() && (filter.len() as i64) >= size.unwrap() {
                    return filter;
                }
            }
        }
        return filter;
    }

    fn find_random(&self, size: Option<i64>) ->  Vec<&Word> {
        let keys = self.map.keys().cloned().collect::<Vec<String>>();
        let mut finish = false;
        let map_len = self.map.len();
        let mut word_vector: Vec<&Word> = vec![];
        while size.is_some() && size.unwrap() > 0 && !finish {
            let mut rng = rand::thread_rng();
            let position = rng.gen_range(0..keys.len());
            
            let key = keys.get(position.clone()).unwrap();
            let word = self.find_visible(key);
            if word.is_some() && !word_vector.iter().any(|e| e.word.eq(&word.unwrap().word)) {
                word_vector.push(word.unwrap())
            }
            if size.is_none() || map_len == word_vector.len() || (word_vector.len() as i64) == size.unwrap() {
                finish = true;
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
            
            if includes.is_none() {
                let mut result = self.find_permute_basic(code.clone(), lax, size_copy);
                code_vector.append(&mut result);
            } else {
                let mut result = self.find_permute_includes(code.clone(), lax, size_copy, includes);
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
            let mut writer = csv::Writer::from_path(self.rebuild_file.clone())?;
    
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
        let mut reader = csv::Reader::from_path(self.source_file.clone())?;
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