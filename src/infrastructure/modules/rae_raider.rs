use scraper::{Html, Selector, ElementRef, CaseSensitivity};
use regex::Regex;
use reqwest;
use reqwest::header::USER_AGENT;

#[allow(dead_code)]
pub struct RaeRaider {
    code: String
}

const ELEMENTS_SELECTOR: &str = "article:first-child .j";
const DESCRIPTION_INDEX: &str = "n_acep";
const TYPE_CODE_I: &str = "d";
const TYPE_CODE_II: &str = "g";
const EXAMPLE_PHRASE: &str = "h";
const REFERENCE: &str = "a";

const BROWSER_AGENT: &str = "Mozilla/5.0";
const DOMAIN: &str = "https://dle.rae.es/";
const QUERY_PARAMS: &str = "m=form";

impl RaeRaider {

    pub fn new(code: String) -> RaeRaider {
        return RaeRaider {
            code
        }
    }

    pub async fn loot_descriptions(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>  {
        let parsed_html = self.html_request().await?;
        
        let mut found = true;
        let mut index = 1;
    
        let mut phrases = vec![];
    
        while found {
            let phrases_level = self.loot_description(parsed_html.clone(), index);

            if phrases_level.as_ref().unwrap().len() == 0 {
                found = false
            } 

            phrases.append(&mut phrases_level.unwrap());

            index += 1;
        }

        Ok(phrases)
    }

    fn loot_description(&self, parsed_html: Html, index: i16) -> Result<Vec<String>, Box<dyn std::error::Error>>  {    
        let mut phrases = vec![];
    
        let mut class_name = ELEMENTS_SELECTOR.to_string();
        if index > 1 {
            class_name = class_name + &index.to_string();
        }

        let selector = &Selector::parse(&class_name).unwrap();
        let elements = parsed_html.select(selector);

        for element in elements {
            let words = element.children()
                .filter_map(|child| ElementRef::wrap(child)
                .filter(|e| self.filter(e)))
                .map(|e| self.to_word(e))
                .filter(|e| !e.is_empty())
                .collect::<Vec<_>>();
    
            let re = Regex::new(r"\.$").unwrap();
            let phrase = re.replace(words.join(" ").trim(), "").to_string();
            phrases.push(phrase);
        }
           
        Ok(phrases)
    }

    fn filter(&self, e: &ElementRef) -> bool {
        return !e.value().has_class(DESCRIPTION_INDEX, CaseSensitivity::CaseSensitive) && 
        !e.value().has_class(TYPE_CODE_I, CaseSensitivity::CaseSensitive) && 
        !e.value().has_class(TYPE_CODE_II, CaseSensitivity::CaseSensitive) && 
        !e.value().has_class(EXAMPLE_PHRASE, CaseSensitivity::CaseSensitive);
    }

    fn to_word(&self, e: ElementRef) -> String {
        let mut word = e.text().next().unwrap().to_string();
        if e.value().has_class(REFERENCE, CaseSensitivity::CaseSensitive) {
            word = "Referencia a '".to_string() + &word + &"':".to_string();
        }
        return word;
    }

    async fn html_request(&self) -> Result<Html, Box<dyn std::error::Error>> {
        let domain = DOMAIN.to_string();
        let query_params = QUERY_PARAMS.to_string();
        let url = domain + &self.code + &"?".to_string() + &query_params;
        let client = reqwest::Client::new();
        let html = client
            .get(url)
            .header(USER_AGENT, BROWSER_AGENT)
            .send().await?.text().await?;
        
        Ok(Html::parse_fragment(html.as_str()))
    }

}