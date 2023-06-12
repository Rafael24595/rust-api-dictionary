use super::configuration;
use crate::configuration::dto_anonymous_collection::DTOAnonymousCollection;
use crate::configuration::dto_collection::DTOCollection;
use crate::configuration::dto_word_lite::DTOWordLite;

use std::time::SystemTime;
use rocket::Rocket;
use rocket::Build;
use rocket::response::status::Custom;
use rocket::http::Status;

pub fn define(build: Rocket<Build>) -> Rocket<Build> {
    build.mount("/word", routes![word])
         .mount("/collection", routes![word_lax])
         .mount("/collection", routes![word_includes])
         .mount("/collection", routes![word_random])
         .mount("/collection", routes![word_permute])
}

#[get("/random?<size>")]
fn word_random(size: Option<i64>) -> Result<String, Custom<String>>{
    let configuration = configuration::get_instance();
    if size.is_some() && size.unwrap() > configuration.max_random {
        return Result::Err(Custom(Status::NotAcceptable, "Cannot process the request, max size for random filter is ".to_string() + &configuration.max_random.to_string() + " elements."));
    }
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let words = configuration::get_instance().word_collection.find_random(size);
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let dtos: Vec<DTOWordLite> = words.iter().map(|word| word.as_dto_lite()).collect();
    let collection = DTOAnonymousCollection{size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: dtos};
    
    return Result::Ok(format!("{}", serde_json::to_string(&collection).unwrap()));
}

#[get("/permute/<combo>?<min>&<exists>&<lax>&<includes>&<size>")]
fn word_permute(combo: &str, min: Option<i8>, exists: Option<bool>, lax: Option<bool>, includes: Option<i8>, size: Option<i64>,) -> Result<String, Custom<String>>{
    let configuration = configuration::get_instance();
    if includes.is_some() && (size.is_none() || size.is_some() && size.unwrap() > configuration.max_permute) {
        return Result::Err(Custom(Status::NotAcceptable, "Cannot process the request, max size for permute filter is ".to_string() + &configuration.max_permute.to_string() + " elements."));
    }
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let words = configuration::get_instance().word_collection.find_permute(&combo.to_string(), min, exists, lax, includes, size);
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let dtos: Vec<DTOWordLite> = words.iter().map(|word| word.as_dto_lite()).collect();
    let collection = DTOAnonymousCollection{size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: dtos};
    
    return Result::Ok(format!("{}", serde_json::to_string(&collection).unwrap()));
}

#[get("/includes/<code>?<position>&<lax>&<size>")]
fn word_includes(code: &str, position: Option<i8>, lax: Option<bool>, size: Option<i64>) -> Result<String, Custom<String>> {
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let key = &code.to_string().to_lowercase();
    let words = configuration::get_instance().word_collection.find_includes(key, position, lax, size);
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let dtos: Vec<DTOWordLite> = words.iter().map(|word| word.as_dto_lite()).collect();
    let collection = DTOCollection{key: code.to_string(), size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: dtos};
    return Result::Ok(format!("{}", serde_json::to_string(&collection).unwrap()));
}

#[get("/lax/<code>")]
fn word_lax(code: &str) -> Result<String, Custom<String>> {
    let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let key = &code.to_string().to_lowercase();
    let words = configuration::get_instance().word_collection.find_lax(key);
    if words.len() == 0 {
        return Result::Err(Custom(Status::NotFound, "Not elements found.".to_string()));
    }
    let finish = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = finish - start;
    let dtos: Vec<DTOWordLite> = words.iter().map(|word| word.as_dto_lite()).collect();
    let collection = DTOCollection{key: code.to_string(), size: words.len(), timestamp: finish.as_millis(), time: time.as_millis(), result: dtos};
    return Result::Ok(format!("{}", format!("{}", serde_json::to_string(&collection).unwrap())));
}

#[get("/<code>")]
fn word(code: &str) -> Result<String, Custom<String>> {
    let key = &code.to_string().to_lowercase();
    let word = configuration::get_instance().word_collection.find(key);
    if word.is_none() {
        return Result::Err(Custom(Status::NotFound, "Not element found.".to_string()));
    }
    return Result::Ok(format!("{}", serde_json::to_string(&word).unwrap()));
}