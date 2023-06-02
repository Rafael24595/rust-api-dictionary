#[path = "../../infrastructure/word_collection_memory.rs"] mod word_collection;

use crate::configuration::word_collection::WordCollection;

pub fn get_collection() -> impl WordCollection {
    return word_collection::WordCollectionMemory::new();
}