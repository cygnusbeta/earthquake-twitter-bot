use std::fs;
use std::collections::HashMap;

fn read_config() {
    let s = fs::read_to_string("config/config.yml")
        .expect("Something went wrong reading the file");
    println!("{}", &s);
    let map: HashMap<String, String> = serde_yaml::from_str(&s).unwrap();
    println!("{:?}", &map);
}

fn main() {
    read_config();
    println!();
}
