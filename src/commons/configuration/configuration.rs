#[path = "../../commons/dependency.rs"] pub(crate) mod dependency;
#[path = "../../domain/word_collection.rs"] pub(crate) mod word_collection;
#[path = "../../infrastructure/dto/dto_anonymous_collection.rs"] pub(crate) mod dto_anonymous_collection;
#[path = "../../infrastructure/dto/dto_collection.rs"] pub(crate) mod dto_collection;
#[path = "../../infrastructure/dto/dto_word.rs"] pub(crate) mod dto_word;
#[path = "../../infrastructure/dto/dto_word_lite.rs"] pub(crate) mod dto_word_lite;
#[path = "../../domain/word.rs"] pub(crate) mod word;
#[path = "diccionary.rs"] pub(crate) mod diccionary;

extern crate unidecode;

use word_collection::WordCollection;
use dependency::Dependency;

use std::{env, time::SystemTime};
use lazy_static::lazy_static;
use std::{sync::Mutex, collections::HashMap};

const SERVICE_NAME: &str = "RUST-DICTIONARY";

pub struct Configuration {
    pub timestamp: u128,
    pub session_id: String,
    pub address: String,
    pub port: u16,
    pub word_collection: Box<dyn WordCollection>
}

lazy_static! {
    static ref INSTANCE: Mutex<Option<Box<Configuration>>> = Mutex::new(None);
}

pub fn load() -> &'static mut Configuration {
    return get_instance();
}

pub fn get_instance() -> &'static mut Configuration {
    let mut lock = INSTANCE.lock().unwrap();

    if lock.is_none() {
        let args = os_env_args();
        let conf = build_configuration(args);

        *lock = Some(Box::new(conf));
    }

    let boxed_conf = lock.as_mut().unwrap();
    let conf_ref = Box::as_mut(boxed_conf);

    unsafe { std::mem::transmute::<&mut Configuration, &'static mut Configuration>(conf_ref) }
}

fn build_configuration(args: HashMap<String, String>) -> Configuration {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    let session_id = SERVICE_NAME.to_string() + "-" + &timestamp.to_string();
    let address = args.get("ROCKET_ADDRESS").unwrap().clone();
    let port = args.get("ROCKET_PORT").unwrap().clone().parse::<u16>().unwrap();

    let mut collection = diccionary::get_collection(args.clone());
    if let Err(e) = collection.on_init() {
        eprintln!("{}", e);
    }
    let word_collection = Box::new(collection);

    return Configuration {
        timestamp,
        session_id,
        address,
        port,
        word_collection
    };
}

fn os_env_args() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, val) in env::vars_os() {
        if let (Ok(k), Ok(v)) = (key.into_string(), val.into_string()) {
            map.insert(k, v);
        }
    }
    return map;
}