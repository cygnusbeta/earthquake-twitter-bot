use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use tokio::runtime::Runtime;

fn read_config() -> HashMap<String, String> {
    let s = fs::read_to_string("config/config.yml")
        .expect("Something went wrong reading the file");
    // println!("{}", &s);
    let map: HashMap<String, String> = serde_yaml::from_str(&s).unwrap();
    // println!("{:?}", &map);
    map
}

fn create_token() -> Token {
    let envs = read_config();

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

async fn tweet(str: String, token: &Token) {
    let post = DraftTweet::new(str.clone()).send(&token).await.unwrap();
}

fn main() {
    let token = create_token();
    let future = tweet("test".to_string(), &token);
    let mut rt = Runtime::new().unwrap();
    rt.block_on(future);
}
