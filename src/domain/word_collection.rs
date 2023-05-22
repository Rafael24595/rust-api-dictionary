use crate::word;
use word::Word;

pub trait WordCollection {
    fn find(&self, code: &String) -> Option<&Word>;
    fn find_includes(&self, code: &String) -> Vec<Option<&Word>>;
    fn insert(&mut self, word: Word);
}