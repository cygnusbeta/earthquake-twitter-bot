use tokio::runtime::Runtime;
use std::fs;
use std::io::Error;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use chrono::{DateTime, FixedOffset};

pub struct FileIO {
    pub fpath: String
}

impl FileIO {
    #[allow(dead_code)]
    pub fn new(fpath: String) -> Self {
        Self {
            fpath: fpath
        }
    }

    #[allow(dead_code)]
    pub fn write(&self, s: String) {
        fs::write(&self.fpath, s).expect("Unable to write file");
    }

    #[allow(dead_code)]
    pub fn init(&self) {
        fs::write(&self.fpath, "").expect("Unable to write file");
    }

    #[allow(dead_code)]
    pub fn append(&self, s: &String) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.fpath)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", s) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

#[allow(dead_code)]
pub fn rt() -> Runtime {
    let rt = Runtime::new().unwrap();
    rt
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
pub fn read_file(filename: String) -> Result<String> {
    let res = fs::read_to_string(filename);
    match res {
        Ok(s) => { return Ok(s) }
        Err(_) => { return Err("Something went wrong reading the file".into()) }
    }
}
