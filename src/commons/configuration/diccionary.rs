#[path = "../../infrastructure/log/logger_console.rs"] mod logger_console;
#[path = "../../infrastructure/word_collection_memory.rs"] mod word_collection_memory;
#[path = "../../infrastructure/modules/rae_raider.rs"] pub(crate) mod rae_raider;
#[path = "../../infrastructure/modules/combo_permuter.rs"] pub(crate) mod combo_permuter;

use std::collections::HashMap;

use crate::configuration::word_collection::WordCollection;
use crate::configuration::logger::Logger;

pub fn get_collection(args: HashMap<String, String>) -> impl WordCollection {
    return word_collection_memory::WordCollectionMemory::new(args);
}

pub fn get_logger(args: HashMap<String, String>) -> impl Logger {
    return logger_console::LoggerConsole::new(args);
}