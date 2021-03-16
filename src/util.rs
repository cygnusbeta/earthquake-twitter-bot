use tokio::runtime::Runtime;


#[allow(dead_code)]
pub fn rt() -> Runtime {
    let rt = Runtime::new().unwrap();
    rt
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
