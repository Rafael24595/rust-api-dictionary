use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CollectionKey<T> {
    pub key: String,
    pub size: usize,
    pub timestamp: u128,
    pub time: u128,
    pub result: Vec<T>
}