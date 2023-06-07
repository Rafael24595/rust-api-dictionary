#[path = "../../infrastructure/word_collection_memory.rs"] mod word_collection;
#[path = "../../infrastructure/modules/rae_raider.rs"] pub(crate) mod rae_raider;
#[path = "../../infrastructure/modules/combo_permuter.rs"] pub(crate) mod combo_permuter;

use std::collections::HashMap;

use crate::configuration::word_collection::WordCollection;

pub fn get_collection(args: HashMap<String, String>) -> impl WordCollection {
    return word_collection::WordCollectionMemory::new(args);
}