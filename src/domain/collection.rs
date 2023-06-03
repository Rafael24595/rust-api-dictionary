use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Collection<T> {
    pub size: usize,
    pub timestamp: u128,
    pub time: u128,
    pub result: Vec<T>
}