#[path = "infrastructure/word_collection_memory.rs"] mod word_collection_memory;
#[path = "domain/word.rs"] mod word;
#[path = "infrastructure/modules/rae_raider.rs"] mod rae_raider;

use crate::word_collection_memory::word_collection::WordCollection;
use crate::word_collection_memory::dependency::Dependency;
use crate::rae_raider::RaeRaider;
use word_collection_memory::WordCollectionMemory;

#[tokio::main]
async fn main() {
    let mut collection = get_collection();
    if let Err(e) = collection.on_init() {
        eprintln!("{}", e);
    }
    println!("{:?}", collection.find(&"Almendralejo".to_string()));
    println!("---------------------------------------------");
    println!("{:?}", collection.find_includes(&"Ab".to_string()));
    println!("---------------------------------------------");
    let random = collection.find_random().unwrap();
    println!("{:?}", random.clone());

    println!("---------------------------------------------");

    let mut raider = RaeRaider::new(random.word.clone().unwrap());

    if raider.load().await.is_ok() {
        let descriptions = raider.loot_descriptions();
    
        if descriptions.as_ref().unwrap().len() == 0 {
            println!("Descriptions not found for word {:?}", random.word.clone().unwrap());
        } 
    
        for description in descriptions.unwrap() {
            println!("{:?}", description);
            println!("-------------------------------------------------------");
        }
    }
}

fn get_collection() -> impl WordCollection {
    return WordCollectionMemory::new();
}