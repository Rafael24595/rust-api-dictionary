use crate::configuration::event_log::EventLog;

pub trait Logger: Send + Sync {
    fn log(&self, event: &EventLog);
}