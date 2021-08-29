use aletheia_hermes::Scraper;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    let target: String =  String::from("https://www.amazon.com/RICA-Listamilk-Vitamin-refrigerated-pasteurized/dp/B087JZQW2R/ref=sr_1_6?_encoding=UTF8&c=ts&dchild=1&keywords=Dairy+Milks&qid=1627806697&s=grocery&sr=1-6&ts_id=6520438011");
    let selector: String = String::from("#priceblock_ourprice");
    let scraper = Scraper::new();
    let job = tokio::spawn(async move { scraper.scrape_website(&target, &selector).await });
    handles.push(job);
    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
        println!("{:?}", results);
    }

    Ok(())
}
