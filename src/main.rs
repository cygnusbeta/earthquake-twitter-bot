use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use chrono::{DateTime, Local};
use util::{Result, read_file, FileIO};
use tweeting::{create_token, tweet_await};
use scraping::{Scraper};
use date::parse_date;
use std::thread::sleep;

#[path = "util.rs"] mod util;
#[path = "tweeting.rs"] mod tweeting;
#[path = "scraping.rs"] mod scraping;
#[path = "date.rs"] mod date;

fn convert_ri(ri: String) -> Result<String> {
    let ri = ri.parse::<f64>()?;
    // ref:
    // 気象庁 | 計測震度の算出方法
    // https://www.data.jma.go.jp/svd/eqev/data/kyoshin/kaisetsu/calc_sindo.htm
    let ri = if ri < 0.5 {
        "0".to_string()
    } else if ri < 1.5 {
        "1".to_string()
    } else if ri < 2.5 {
        "2".to_string()
    } else if ri < 3.5 {
        "3".to_string()
    } else if ri < 4.5 {
        "4".to_string()
    } else if ri < 5.0 {
        "5弱".to_string()
    } else if ri < 5.5 {
        "5強".to_string()
    } else if ri < 6.0 {
        "6弱".to_string()
    } else if ri < 6.5 {
        "6強".to_string()
    } else {
        "7".to_string()
    };
    Ok(ri)
}

fn write_date_last(date: String) {
    let f_date_last = FileIO::new("out/date_last.txt".to_string());
    f_date_last.write(date);
    println!("Saved date on page to `date_last.txt`");
}

fn init() -> Result<()> {
    let scraper = Scraper::fetch("http://acrs.sci.ibaraki.ac.jp/".to_string())?;
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string())?;

    write_date_last(date);
    println!("`date_last.txt` initialized.");
    Ok(())
}

fn read_and_parse_file(fpath: String) -> Result<DateTime<Local>> {
    let s = read_file(fpath)?;
    let date_last = parse_date(s)?;
    Ok(date_last)
}

fn try_run() -> Result<()> {
    let date_last = match read_and_parse_file("out/date_last.txt".to_string()) {
        Ok(date) => {
            println!("date_last: {}", date.format("%Y/%m/%d %H:%M:%S").to_string());
            date
        }
        Err(e) => {
            eprintln!("`date_last.txt` is not found or corrupted. Initializing.");
            eprintln!("{}", e);
            init()?;
            return Err(e);
        }
    };

    let scraper = Scraper::fetch("http://acrs.sci.ibaraki.ac.jp/".to_string())?;
    let date = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(1) > strong".to_string())?;
    let ri = scraper.select("body > table > tbody > tr > td > div:nth-child(3) > ul > li:nth-child(2) > strong:nth-child(1)".to_string())?;

    let date = parse_date(date)?;
    println!("date on page: {}", date.format("%Y/%m/%d %H:%M:%S").to_string());
    if date - date_last < chrono::Duration::seconds(1) {
        // if date on page is not refreshed
        println!("date on page == `date_last.txt`. Not tweeting.");
    } else {
        // if date on page is refreshed
        println!("date on page != `date_last.txt`");

        // Save date on page to `date_last.txt`
        let s = date.format("%Y/%m/%d, %H:%M:%S").to_string();
        write_date_last(s);

        // Tweet
        let token = create_token("config/config.yml".to_string());
        let ri = convert_ri(ri)?;
        /*
            【地震観測情報】10時28分頃、地震を観測しました。

            観測日時：2021年05月01日 10時28分11秒
            水戸キャンパス震度：震度1
         */
        let body = format!("【地震観測情報】{}頃、地震を観測しました。\n\n観測日時：{}\n水戸キャンパス震度：震度{}",
                           date.format("%H時%M分"), date.format("%Y年%m月%d日 %H時%M分%S秒"), ri);
        tweet_await(body, &token)?;
    }
    Ok(())
}

fn run() {
    println!("[{}] Cron job started.", Local::now().format("%Y/%m/%d %H:%M:%S").to_string());
    match try_run() {
        Ok(_) => {},
        Err(e) => println!("{}", e)
    }
    println!("[{}] Cron job ended.", Local::now().format("%Y/%m/%d %H:%M:%S").to_string());
}

fn main() {
    init().unwrap();
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