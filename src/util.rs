use tokio::runtime::Runtime;

pub fn rt() -> Runtime {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt
}
