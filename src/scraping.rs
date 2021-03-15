use reqwest;
use scraper::{Html, Selector};

#[path = "util.rs"] mod util;

struct Scraper {
    body: String
}

impl Scraper {
    fn fetch(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        println!("scraping... {}", &url);
        let resp = reqwest::blocking::get(&url)?;
        println!("-> response: `{}`", &resp.status());
        assert!(resp.status().is_success());
        let body = resp.text()?;
        Ok(Self {
            body: body
        })
    }

    fn select(&self, selector: String) -> Result<String, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.body);
        let selector = Selector::parse(selector.as_str()).unwrap();
        let elements = document.select(&selector);
        let text = elements.map(|e| format!("{}", e.text().collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join(" ");
        assert!(!&text.is_empty(), "selector not found");
        println!("   scraped text: `{}`", &text);
        Ok(text)
    }
}



fn get_ri() {
    let scraper = Scraper::fetch("http://157.80.67.225/".to_string()).unwrap();
    let text = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(2) > strong:nth-child(1)".to_string()).unwrap();
}

fn main() {
    get_ri();
}
