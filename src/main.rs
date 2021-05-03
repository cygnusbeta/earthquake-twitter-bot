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
use tweeting::{create_token, tweet};
use scraping::{Scraper};

#[path = "util.rs"] mod util;
#[path = "tweeting.rs"] mod tweeting;
#[path = "scraping.rs"] mod scraping;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("I get executed every 10 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}