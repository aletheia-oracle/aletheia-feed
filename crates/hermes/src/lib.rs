use csv::Writer;
use scraper::Html;
use scraper::Selector;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::fs::OpenOptions;
use reqwest;
use url::Url;

pub struct Parser {
    client: reqwest::Client
}

impl Parser {
    pub fn init_parser(&mut self){
        self.client = reqwest::Client::new();
    }
    pub async fn scrape_website(&self, url: &str, selector: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = Url::parse(&url)?;
        let response = self.client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&response);
        let selector = Selector::parse(&selector).unwrap();
        for element in document.select(&selector) {
            println!("element: {:?}", element.inner_html())
        }
        Ok(())
            
    }
}
