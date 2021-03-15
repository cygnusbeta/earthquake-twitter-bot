use reqwest;
use tokio::runtime::Runtime;
use std::future::Future;
use std::error::Error;
use reqwest::Response;
use crate::util::rt;

#[path = "util.rs"] mod util;

fn get_ri() {
    let url = "https://www.google.com/".to_string();
    let resp = rt().block_on(async {
        let mut resp = reqwest::get(url).await.unwrap();
        resp
    });
    println!("{}", &resp.status());
    assert!(resp.status().is_success());
}

fn main() {
    get_ri();
}