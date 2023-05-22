use crate::word;
use super::dependency::Dependency;
use word::Word;

pub trait WordCollection: Dependency {
    fn find(&self, code: &String) -> Option<&Word>;
    fn find_includes(&self, code: &String) -> Vec<Option<&Word>>;
}