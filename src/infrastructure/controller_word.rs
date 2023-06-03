use super::configuration;
use crate::configuration::collection::Collection;
use crate::configuration::collection_key::CollectionKey;
use std::time::SystemTime;

use rocket::Rocket;
use rocket::Build;

use rocket::response::status;

pub fn define(build: Rocket<Build>) -> Rocket<Build> {
    build.mount("/word", routes![word])
         .mount("/word", routes![word_includes])
         .mount("/word", routes![word_random])
}

#[get("/random?<size>")]
fn word_random(size: Option<i64>) -> status::Accepted<String> {
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let words = configuration::get_instance().word_collection.find_random(size);
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let collection = Collection{size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: words};
    status::Accepted(Some(format!("{}", serde_json::to_string(&collection).unwrap())))
}

#[get("/includes/<code>?<size>")]
fn word_includes(code: &str, size: Option<i64>) -> status::Accepted<String> {
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let key = &code.to_string().to_lowercase();
    let words = configuration::get_instance().word_collection.find_includes(key, size);
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let collection = CollectionKey{key: code.to_string(), size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: words};
    status::Accepted(Some(format!("{}", serde_json::to_string(&collection).unwrap())))
}

#[get("/<code>")]
fn word(code: &str) -> status::Accepted<String> {
    let key = &code.to_string().to_lowercase();
    let word = configuration::get_instance().word_collection.find(key);
    status::Accepted(Some(format!("{}", serde_json::to_string(&word).unwrap())))
}