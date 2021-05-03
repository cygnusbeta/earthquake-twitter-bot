use reqwest;
use scraper::{Html, Selector};
use util::Result;

#[path = "util.rs"] mod util;

pub struct Scraper {
    document: Html
}

impl Scraper {
    pub fn fetch(url: String) -> Result<Self> {
        println!("scraping... {}", &url);
        let resp = reqwest::blocking::get(&url)?;
        println!("-> response: `{}`", &resp.status());
        assert!(resp.status().is_success());
        let body = resp.text()?;
        let document = Html::parse_document(&body);
        Ok(Self {
            document: document
        })
    }

    pub fn select(&self, selector: String) -> Result<String> {
        let selector = Selector::parse(selector.as_str()).unwrap();
        let elements = self.document.select(&selector);
        let text = elements.map(|e| format!("{}", e.text().collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join(" ");
        assert!(!&text.is_empty(), "selector not found");
        println!("   scraped text: `{}`", &text);
        Ok(text)
    }
}

#[allow(dead_code)]
fn main() {
    let scraper = Scraper::fetch("http://157.80.67.225/".to_string()).unwrap();
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string()).unwrap();
    let ri = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(2) > strong:nth-child(1)".to_string()).unwrap();
}
