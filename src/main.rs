use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use egg_mode::media::{upload_media, media_types};
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Local};
use chrono::format::ParseError;
use util::{rt, Result, read_file, FileIO};
use tweeting::{create_token, tweet, tweet_w_img};
use scraping::{Scraper};
use date::parse_date;
use std::thread::sleep;

#[path = "util.rs"] mod util;
#[path = "tweeting.rs"] mod tweeting;
#[path = "scraping.rs"] mod scraping;
#[path = "date.rs"] mod date;

fn write_date_last(date: String) {
    let f_date_last = FileIO::new("out/date_last.txt".to_string());
    f_date_last.write(date);
    println!("date_last refreshed.");
}

fn init() {
    let scraper = Scraper::fetch("http://157.80.67.225/".to_string()).unwrap();
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string()).unwrap();

    write_date_last(date);
    println!("`date_last.txt` initialized.");
}

fn read_and_parse_file(date: String) -> Result<DateTime<Local>> {
    let s = read_file("out/date_last.txt".to_string())?;
    let date_last = parse_date(s)?;
    Ok(date_last)
}



fn run() {
    let mut date_last: DateTime<Local>;
    match read_and_parse_file("out/date_last.txt".to_string()) {
        Ok(date) => {
            date_last = date;
            println!("date_last: {}", &date_last)
        }
        Err(e) => {
            println!("`date_last.txt` is not found or corrupted. Initializing.");
            println!("{}", e);
            init();
            return;
        }
    }
    let date_last = date_last;

    let scraper = Scraper::fetch("http://157.80.67.225/".to_string()).unwrap();
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string()).unwrap();
    let ri = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(2) > strong:nth-child(1)".to_string()).unwrap();

    let date = parse_date(date).unwrap();
    if date - date_last < chrono::Duration::seconds(60) {
        println!("Time is not refreshed. Not tweeting.");
    } else {
        let token = create_token("config/config.yml".to_string());
        let body = format!("{}: {}", date, ri);
        rt().block_on(async {
            // tweet("test2".to_string(), &token).await.unwrap();
            tweet(body, &token).await.unwrap();
        });

        let s = date.format("%Y/%m/%d, %H:%M:%S").to_string();
        write_date_last(s)
    }
}

fn main() {
    init();
    println!("Wating for 1 minute.");
    sleep(Duration::from_secs(60));

    let mut sched = JobScheduler::new();

    // Run every minute
    sched.add(Job::new("0 * * * * *".parse().unwrap(), || {
        run();
    }));

    println!("Scheduler started.");
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}