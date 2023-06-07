use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;

pub trait WordCollection: Dependency {
    fn find(&mut self, code: &String) -> Option<&Word>;
    fn find_includes(&self, code: &String, position: Option<i8>, size: Option<i64>) -> Vec<&Word>;
    fn find_random(&self, size: Option<i64>) ->  Vec<&Word>;
    fn find_permute(&mut self, combo: &String, size: Option<i64>, exists: Option<bool>) -> Vec<Word>;
}