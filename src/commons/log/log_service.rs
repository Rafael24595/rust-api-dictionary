use crate::configuration::logger::Logger;

use lazy_static::lazy_static;
use std::{sync::Mutex};

use super::event_log::EventLog;

pub struct LogService {
    logger: Box<dyn Logger> 
}

lazy_static! {
    static ref INSTANCE: Mutex<Option<Box<LogService>>> = Mutex::new(None);
}

pub fn load(logger: impl Logger + 'static) -> &'static mut LogService {
    let mut lock = INSTANCE.lock().unwrap();

    if lock.is_none() {
        let logger_box = Box::new(logger);
        let log = LogService { logger: logger_box };

        *lock = Some(Box::new(log));
    }

    let boxed_log = lock.as_mut().unwrap();
    let log_ref = Box::as_mut(boxed_log);

    unsafe { std::mem::transmute::<&mut LogService, &'static mut LogService>(log_ref) }
}

pub fn get_instance() -> &'static mut LogService {
    let mut lock = INSTANCE.lock().unwrap();
    let boxed_log = lock.as_mut().unwrap();
    let log_ref = Box::as_mut(boxed_log);

    unsafe { std::mem::transmute::<&mut LogService, &'static mut LogService>(log_ref) }
}

impl LogService {
    
    pub fn log(&self, category: &str, family: &str, message: &str) {
        let event = EventLog::new(category.to_string(), family.to_string(), message.to_string());
        self.logger.log(&event);
    }
    

}