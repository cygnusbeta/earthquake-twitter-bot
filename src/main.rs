use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use egg_mode::media::{upload_media, media_types};
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use util::{rt, Result};
use tweeting::{create_token, tweet, tweet_w_img};
use scraping::{Scraper};

#[path = "util.rs"] mod util;
#[path = "tweeting.rs"] mod tweeting;
#[path = "scraping.rs"] mod scraping;

fn run() {
    let scraper = Scraper::fetch("http://157.80.67.225/".to_string()).unwrap();
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string()).unwrap();
    let ri = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(2) > strong:nth-child(1)".to_string()).unwrap();

    // let token = create_token("config/config.yml".to_string());
    // rt().block_on(async {
    //     // tweet("test2".to_string(), &token).await.unwrap();
    //     tweet_w_img("test w/img".to_string(), "assets/test/test.png".to_string(), &token).await.unwrap();
    // });
}

fn main() {
    let mut sched = JobScheduler::new();

    // Run every minute
    sched.add(Job::new("0 * * * * *".parse().unwrap(), || {
        run();
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}