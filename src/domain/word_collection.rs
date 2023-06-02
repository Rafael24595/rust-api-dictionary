use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;

pub trait WordCollection: Dependency {
    fn find(&self, code: &String) -> Option<&Word>;
    fn find_includes(&self, code: &String) -> Vec<Option<&Word>>;
    fn find_random(&self) -> Option<&Word>;
}