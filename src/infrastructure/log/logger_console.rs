use std::collections::HashMap;

use crate::configuration::logger::Logger;
use crate::configuration::event_log::EventLog;

#[allow(dead_code)]
pub struct LoggerConsole {
}

unsafe impl Send for LoggerConsole{}
unsafe impl Sync for LoggerConsole{}

impl LoggerConsole {

    pub fn new(args: HashMap<String, String>) -> impl Logger {
        return LoggerConsole {}
    }

}

impl Logger for LoggerConsole {

    fn log(&self, event: &EventLog) {
        println!("{} - [{}] -> [{}]: {}", event.timestamp.to_string(), event.category, event.family, event.message);
    }

}