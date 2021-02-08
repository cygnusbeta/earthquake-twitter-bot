use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use tokio::runtime::Runtime;
use egg_mode::media::{upload_media, media_types};
use std::fs::File;
use std::io::Read;

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
    println!("Tweeting...");
    let post = DraftTweet::new(str.clone()).send(&token).await.unwrap();
    let user = post.response.user.unwrap();
    println!("Successfully tweeted:");
    println!("@{} `{}`: `{}`", &user.screen_name, &user.name, &post.response.text)
}

async fn tweet_w_img(str: String, img_path: String, token: &Token) {
    let mut image = Vec::new();
    {
        let mut file = File::open(img_path.clone()).unwrap();
        file.read_to_end(&mut image).unwrap();
    }
    let image = image;

    let image_fname = img_path.clone().split('/').collect::<Vec<_>>().last().unwrap().to_string();
    println!("Uploading image: `{}`...", &image_fname);
    let handle = upload_media(&image, &media_types::image_png(), &token).await.unwrap();
    println!("Successfully uploaded.");

    println!("Tweeting...");
    let mut draft = DraftTweet::new(str.clone());
    draft.add_media(handle.id);
    let post = draft.send(&token).await.unwrap();

    let user = post.response.user.unwrap();
    println!("Successfully tweeted:");
    println!("@{} `{}`: `{}` (image: `{}`)", &user.screen_name, &user.name, &post.response.text, &image_fname);
}

fn main() {
    let token = create_token();
    // let future = tweet("test2".to_string(), &token);
    let future = tweet_w_img("test w/img".to_string(), "assets/test/test.png".to_string(), &token);
    let mut rt = Runtime::new().unwrap();
    rt.block_on(future);
}
