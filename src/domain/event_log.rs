use std::time::SystemTime;

use serde::Deserialize;
use serde::Serialize;

use crate::configuration;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct EventLog {
    pub register_id: Option<i128>,
	pub session_id: String,
	pub category: String,
	pub family: String,
	pub message: String,
	pub timestamp: u128,
}

impl EventLog {

    pub fn new(category: String, family: String, message: String) -> EventLog {
        let register_id = Option::None;
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let session_id = configuration::get_instance().session_id.clone();
        return EventLog {
            register_id,
            session_id,
            category,
            family,
            message,
            timestamp
        };
    }

}