use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;

fn read_config() -> HashMap<String, String> {
    let s = fs::read_to_string("config/config.yml")
        .expect("Something went wrong reading the file");
    println!("{}", &s);
    let map: HashMap<String, String> = serde_yaml::from_str(&s).unwrap();
    println!("{:?}", &map);
    map
}

fn create_token(envs: &HashMap<String, String>) -> Token {
    let consumer_key = envs["consumer_key"].clone();
    let consumer_secret = envs["consumer_secret"].clone();
    let access_token_key = envs["access_token_key"].clone();
    let access_token_secret = envs["access_token_secret"].clone();

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    token
}

async fn tweet(envs: &HashMap<String, String>) {
    let token = create_token(&envs);
    // let post = DraftTweet::new("Hey Twitter!").send(&token).await.unwrap();
}

fn main() {
    let envs = read_config();
    tweet(&envs);
}
