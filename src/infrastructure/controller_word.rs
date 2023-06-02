use super::configuration;

use rocket::Rocket;
use rocket::Build;

use rocket::response::status;

pub fn define(build: Rocket<Build>) -> Rocket<Build> {
    build.mount("/word", routes![word])
         .mount("/word", routes![word_includes])
         .mount("/word", routes![word_random])
}

#[get("/random")]
fn word_random() -> status::Accepted<String> {
    let word = configuration::get_instance().word_collection.find_random();
    status::Accepted(Some(format!("id: '{}'", serde_json::to_string(&word).unwrap())))
}

#[get("/includes/<code>")]
fn word_includes(code: &str) -> status::Accepted<String> {
    let key = &code.to_string().to_lowercase();
    let word = configuration::get_instance().word_collection.find_includes(key);
    status::Accepted(Some(format!("id: '{}'", serde_json::to_string(&word).unwrap())))
}

#[get("/<code>")]
fn word(code: &str) -> status::Accepted<String> {
    let key = &code.to_string().to_lowercase();
    let word = configuration::get_instance().word_collection.find(key);
    status::Accepted(Some(format!("id: '{}'", serde_json::to_string(&word).unwrap())))
}