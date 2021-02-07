use std::fs;

fn read_config() {
    let config = fs::read_to_string("config/config.yml")
        .expect("Something went wrong reading the file");
    println!("{}", &config);
}

fn main() {
    read_config();
    println!("Hello, world!");
}
