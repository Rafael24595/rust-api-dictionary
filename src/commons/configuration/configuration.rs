#[path = "../../commons/dependency.rs"] pub(crate) mod dependency;
#[path = "../../domain/word_collection.rs"] pub(crate) mod word_collection;
#[path = "../../domain/collection.rs"] pub(crate) mod collection;
#[path = "../../domain/collection_key.rs"] pub(crate) mod collection_key;
#[path = "../../domain/word.rs"] pub(crate) mod word;
#[path = "diccionary.rs"] mod diccionary;

use word_collection::WordCollection;
use dependency::Dependency;

use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Configuration {
    pub word_collection: Box<dyn WordCollection>
}

lazy_static! {
    static ref INSTANCE: Mutex<Option<Box<Configuration>>> = Mutex::new(None);
}

pub fn get_instance() -> &'static mut Configuration {
    let mut lock = INSTANCE.lock().unwrap();

    if lock.is_none() {
        let mut collection = diccionary::get_collection();

        if let Err(e) = collection.on_init() {
            eprintln!("{}", e);
        }

        let word_collection = Box::new(collection);

        let conf = Configuration {
            word_collection
        };

        *lock = Some(Box::new(conf));
    }

    let boxed_conf = lock.as_mut().unwrap();
    let conf_ref = Box::as_mut(boxed_conf);

    unsafe { std::mem::transmute::<&mut Configuration, &'static mut Configuration>(conf_ref) }
}