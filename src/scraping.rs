use reqwest;
use scraper::{Html, Selector};

#[path = "util.rs"] mod util;

fn scrape(url: String, selector: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("scraping... {}", &url);
    let resp = reqwest::blocking::get(url)?;
    println!("-> response: `{}`", &resp.status());
    assert!(resp.status().is_success());
    let body = resp.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(selector.as_str()).unwrap();
    let elements = document.select(&selector);
    let text = elements.map(|e| format!("{}", e.text().collect::<Vec<_>>().join(" "))).collect::<Vec<_>>().join(" ");
    assert!(!&text.is_empty(), "selector not found");
    println!("   scraped text: `{}`", &text);
    Ok(text)
}

fn get_ri() {
    scrape(
        "http://evrrss.eri.u-tokyo.ac.jp/db/index.html".to_string(),
        "body > font > blockquote > h1".to_string()
    ).unwrap();
}

fn main() {
    get_ri();
}
