use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;

pub trait WordCollection: Dependency {
    fn find(&mut self, code: &String) -> Option<&Word>;
    fn find_includes(&self, code: &String, size: Option<i64>) -> Vec<&Word>;
    fn find_random(&self, size: Option<i64>) ->  Vec<&Word>;
}