use csv::Writer;
use scraper::Html;
use scraper::Selector;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::fs::OpenOptions;
use tokio;
use reqwest;
use url::Url;

struct Parser {
    client: reqwest::Client
    
}

impl Parser {
    async fn init_parser(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.client = reqwest::Client::new();
        Ok(())
    }
    async fn scrape_website(&self, url: &str, selector: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    let client = reqwest::Client::new();
    let job = tokio::spawn(async move { scrape_website().await });
    handles.push(job);

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }

    Ok(())
}
