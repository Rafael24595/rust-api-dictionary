#[path = "infrastructure/controller_word.rs"] mod controller_word;
#[path = "commons/configuration/configuration.rs"] mod configuration;

#[macro_use] extern crate rocket;

use dotenv::dotenv;

#[rocket::main]
async fn main() {
    dotenv().ok();
    
    let _ = on_init();
    let _ = serve().await;
    let _ = on_exit();
}

fn on_init() {
    configuration::load();
}

async fn serve() {
    let configuration = configuration::get_instance();

    let figment = rocket::Config::figment()
        .merge(("address", configuration.address.clone()))
        .merge(("port", configuration.port.clone()));

    let mut build = rocket::custom(figment);
    build = controller_word::define(build);
    let _ = build.launch().await;
}

fn on_exit() {
    let config = configuration::get_instance();
    let _ = config.word_collection.on_exit();
}