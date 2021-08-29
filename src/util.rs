use tokio::runtime::Runtime;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;

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
pub fn read_file(fpath: String) -> Result<String> {
    let res = fs::read_to_string(fpath);
    match res {
        Ok(s) => { return Ok(s) }
        Err(_) => { return Err("Something went wrong reading the file".into()) }
    }
}


// use: err("custome name error".to_string());
#[allow(dead_code)]
pub fn err<T>(str: String) -> std::result::Result<T, std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, str))
}

// use: err_custom("custome name error".to_string(), ErrorKind::InvalidData);
#[allow(dead_code)]
pub fn err_custom<T>(str: String, errorkind: std::io::ErrorKind) -> std::result::Result<T, std::io::Error> {
    Err(std::io::Error::new(errorkind, str))
}

// assert_err_msg(condition, error_msg)?;
// use: assert_err_msg(condition, "invalid data")?;
#[allow(dead_code)]
pub fn assert_err_msg<T: std::fmt::Display + std::default::Default>(condition: bool, error_msg: T) -> std::result::Result<T, std::io::Error> {
    if condition {
        let res: T = Default::default();
        Ok(res)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("assertion failed: {}", error_msg)))
    }
}

// assert_err(condition)?;
// use: assert_err(condition)?;
#[allow(dead_code)]
pub fn assert_err<T: std::default::Default>(condition: bool) -> std::result::Result<T, std::io::Error> {
    if condition {
        let res: T = Default::default();
        Ok(res)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("assertion failed")))
    }
}
