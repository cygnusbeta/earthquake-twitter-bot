use util::{Result};
use chrono::{DateTime, Local};

#[path = "util.rs"] mod util;

pub fn parse_date(date: String) -> Result<DateTime<Local>> {
    let tz_local = Local::now().format("%z").to_string(); // "+0900"
    // println!("{}", tz_local);
    let date = format!("{} {}", date, tz_local);
    // let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    let custom = DateTime::parse_from_str(date.as_str(), "%Y/%m/%d, %H:%M:%S %z")?;
    // println!("{}", custom);
    let custom = custom.with_timezone(&Local);
    Ok(custom)
}

#[allow(dead_code)]
fn main() {
    parse_date("2021/05/01, 10:28:11".to_string()).unwrap();
}
