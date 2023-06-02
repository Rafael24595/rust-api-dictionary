use std::error::Error;

pub trait Dependency: Send + Sync {
    fn on_init(&mut self) -> Result<(), Box<dyn Error>>;
    fn on_exit(&mut self) -> Result<(), Box<dyn Error>>;
}