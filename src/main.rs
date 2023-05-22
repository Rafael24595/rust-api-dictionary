#[path = "domain/word_collection_memory.rs"] mod word_collection_memory;
#[path = "domain/word.rs"] mod word;

use crate::word_collection_memory::word_collection::WordCollection;
use crate::word_collection_memory::dependency::Dependency;
use word_collection_memory::WordCollectionMemory;

fn main() {
    let mut collection = get_collection();
    if let Err(e) = collection.on_init() {
        eprintln!("{}", e);
    }
    println!("{:?}", collection.find(&"Almendralejo".to_string()));
    println!("---------------------------------------------");
    println!("{:?}", collection.find_includes(&"Ab".to_string()));
}

fn get_collection() -> impl WordCollection + Dependency {
    return WordCollectionMemory::new();
}