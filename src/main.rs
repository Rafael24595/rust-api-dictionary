#[path = "infrastructure/controller_word.rs"] mod controller_word;
#[path = "commons/configuration/configuration.rs"] mod configuration;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    configuration::get_instance();
    let build = rocket::build();
    controller_word::define(build)
}