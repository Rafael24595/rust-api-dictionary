#[path = "infrastructure/controller_word.rs"] mod controller_word;
#[path = "commons/configuration/configuration.rs"] mod configuration;
#[macro_use] extern crate rocket;
use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();
    
    let config = configuration::get_instance();
    let build = rocket::build();
    let _ = controller_word::define(build)
        .launch()
        .await;

    let _ = config.word_collection.on_exit();
}