use scraper::{Html, Selector, ElementRef, CaseSensitivity, Node};
use ego_tree::{NodeRef};
use tokio::task::block_in_place;
use reqwest;
use reqwest::header::USER_AGENT;
use regex::Regex;

#[allow(dead_code)]
pub struct RaeRaider {
    code: String,
    html: Option<Html>
}

const ELEMENTS_SELECTOR: &str = "article:first-child .j";
const DESCRIPTION_INDEX: &str = "n_acep";
const TYPE_CODE_I: &str = "d";
const TYPE_CODE_II: &str = "g";
const TYPE_CODE_III: &str = "c";
const EXAMPLE_PHRASE: &str = "h";
const REFERENCE: &str = "a";

const BROWSER_AGENT: &str = "Mozilla/5.0";
const DOMAIN: &str = "https://dle.rae.es/";
const QUERY_PARAMS: &str = "m=form";

impl RaeRaider {

    pub fn new(code: String) -> RaeRaider {
        return RaeRaider {
            code,
            html: Option::None
        }
    }

    pub fn load(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut status = false;
        if self.html.is_none() {
            let req = self.html_request();
            self.html = Option::Some(req.unwrap());
            status = true;
        }
        Ok(status)
    }

    pub fn loot_descriptions(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if self.html.is_none() {
            panic!("HTML is not loaded.");
        } 

        let parsed_html = self.html.as_ref().unwrap();
    
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
                .filter(|e| self.filter(e))
                .map(|e| self.element_to_string(e))
                .filter(|e| !e.is_empty())
                .collect::<Vec<_>>();
    
            let re = Regex::new(r"\.$").unwrap();
            let phrase = re.replace(words.join("").trim(), "").to_string();
            phrases.push(phrase);
        }
           
        Ok(phrases)
    }

    fn filter(&self, e: &NodeRef<Node>) -> bool {
        if e.value().is_text() {
            return true;
        }
        return 
            !e.value().as_element().unwrap().has_class(DESCRIPTION_INDEX, CaseSensitivity::CaseSensitive) && 
            !e.value().as_element().unwrap().has_class(TYPE_CODE_I, CaseSensitivity::CaseSensitive) && 
            !e.value().as_element().unwrap().has_class(TYPE_CODE_II, CaseSensitivity::CaseSensitive) && 
            !e.value().as_element().unwrap().has_class(TYPE_CODE_III, CaseSensitivity::CaseSensitive) && 
            !e.value().as_element().unwrap().has_class(EXAMPLE_PHRASE, CaseSensitivity::CaseSensitive);
    }

    fn element_to_string(&self, e: NodeRef<Node>) -> String {
        let mut word = String::new();
        let node = e.value();

        if node.is_text() {
            let text = node.as_text().unwrap();
            return text.to_string();
        }

        if node.is_element() {
            let mut text = ElementRef::wrap(e).unwrap().text();
            let element = node.as_element().unwrap();
            word = text.next().unwrap().to_string() ;

            if element.has_class(REFERENCE, CaseSensitivity::CaseSensitive) {
                word = "Referencia a ".to_string() + &word ;
            }
        }

        return word;
    }

    fn html_request(&self) -> Result<Html, Box<dyn std::error::Error>> {
        let domain = DOMAIN.to_string();
        let query_params = QUERY_PARAMS.to_string();
        let url = format!("{}{}?{}", domain, self.code, query_params);

        let html = block_in_place::<_, Result<_, reqwest::Error>>(|| {
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&url)
                .header(USER_AGENT, BROWSER_AGENT)
                .send()?;
            let html = response.text()?;
            Ok(Html::parse_fragment(&html))
        })?;

        Ok(html)
    }

    
}