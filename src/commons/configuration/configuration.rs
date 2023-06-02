#[path = "../../commons/dependency.rs"] pub(crate) mod dependency;
#[path = "../../domain/word_collection.rs"] pub(crate) mod word_collection;
#[path = "../../domain/word.rs"] pub(crate) mod word;
#[path = "diccionary.rs"] mod diccionary;

use word_collection::WordCollection;
use dependency::Dependency;

pub struct Configuration {
    pub word_collection: Box<dyn WordCollection>
}

static mut INSTANCE: Option<Box<Configuration>> = Option::None;

pub fn get_instance() -> &'static Configuration {
    if unsafe { INSTANCE.is_none() } {
        let mut collection = diccionary::get_collection();

        if let Err(e) = collection.on_init() {
            eprintln!("{}", e);
        }

        let word_collection = Box::new(collection);

        let conf = Configuration {
            word_collection
        };
        
        unsafe { INSTANCE = Option::Some(Box::new(conf)) };
    }
    return unsafe { INSTANCE.as_ref().unwrap() };
}