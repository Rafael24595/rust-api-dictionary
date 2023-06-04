#[path = "infrastructure/controller_word.rs"] mod controller_word;
#[path = "commons/configuration/configuration.rs"] mod configuration;

#[macro_use] extern crate rocket;

use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();
    
    configuration::load();
    let mut build = rocket::build();
    build = controller_word::define(build);
    let _ = build.launch().await;

    on_exit();
}

fn on_exit() {
    let config = configuration::get_instance();
    let _ = config.word_collection.on_exit();
}