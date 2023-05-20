use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
//TODO: Rewrite.
struct Word {
    word: Option<String>,
    category: Option<String>,
    genre: Option<String>,
    number: Option<i64>,
    root: Option<String>,
    affix: Option<String>, 
    tonic: Option<String>, 
    syllables: Option<String>, 
    locale: Option<String>, 
    origin: Option<String>, 
    synonyms: Option<String>
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("Headers");
    println!("{:?}", headers);
    println!("---------------------------------------------");
    println!("Body");
    for result in reader.deserialize() {
        let record: Word = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./src/assets/Dictionary_es.csv") {
        eprintln!("{}", e);
    }
}