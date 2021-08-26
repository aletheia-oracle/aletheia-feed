use csv::Writer;
use reqwest;
use scraper::Html;
use scraper::Selector;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::fs::OpenOptions;
use url::Url;

pub struct Scraper {
    client: reqwest::Client,
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    pub async fn scrape_website(
        &self,
        url: &str,
        selector: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
