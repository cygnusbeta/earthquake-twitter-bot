use std::fs;
use std::collections::HashMap;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;
use egg_mode::media::{upload_media, media_types};
use std::fs::File;
use std::io::Read;
use util::{rt, Result};
use twitter_v2::TwitterApi;
use twitter_v2::authorization::Oauth1aToken;

#[path = "util.rs"] mod util;

fn read_config(config_path: String) -> HashMap<String, String> {
    let s = fs::read_to_string(config_path)
        .expect("Something went wrong reading the file");
    // println!("{}", &s);
    let map: HashMap<String, String> = serde_yaml::from_str(&s).unwrap();
    // println!("{:?}", &map);
    map
}

pub fn create_token(config_path: String) -> Oauth1aToken {
    let envs = read_config(config_path);

    let consumer_key = envs["consumer_key"].clone();
    let consumer_secret = envs["consumer_secret"].clone();
    let access_token_key = envs["access_token_key"].clone();
    let access_token_secret = envs["access_token_secret"].clone();
    let token: Oauth1aToken = Oauth1aToken::new(consumer_key, consumer_secret,
                                                access_token_key, access_token_secret);
    token
}

#[allow(dead_code)]
pub async fn tweet(body: String, token: &Oauth1aToken) -> Result<()> {
    println!("Tweeting...");

    let tweet = TwitterApi::new(token.clone())
        .post_tweet()
        .text(body)
        .send()
        .await?
        .into_data()
        .expect("this tweet should exist");
    // let user = post.response.user.unwrap();

    // println!("Successfully tweeted:");
    // println!("@{} `{}`: `{}`", &user.screen_name, &user.name, &tweet.text);
    println!("Successfully tweeted: `{}`", &tweet.text);
    // println!("https://twitter.com/{}/status/{}", , tweet.id);
    Ok(())
}

#[allow(dead_code)]
pub fn tweet_await(body: String, token: &Oauth1aToken) -> Result<()> {
    let res = rt().block_on(async {
        // tweet("test2".to_string(), &token).await.unwrap();
        let res = tweet(body, &token).await;
        let res = match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
        res
    });
    res
}

#[allow(dead_code)]
fn read_img(img_path: &String) -> Vec<u8> {
    let mut image = Vec::new();
    {
        let mut file = File::open(img_path.clone()).unwrap();
        file.read_to_end(&mut image).unwrap();
    }
    image
}

#[allow(dead_code)]
pub async fn tweet_w_img(body: String, img_path: String, token: &Token) -> Result<()> {
    let image_fname = img_path.clone().split('/').collect::<Vec<_>>().last().unwrap().to_string();
    println!("Uploading image: `{}`...", &image_fname);
    let handle = upload_media(&read_img(&img_path), &media_types::image_png(), &token).await?;
    println!("Successfully uploaded.");

    println!("Tweeting...");
    let mut draft = DraftTweet::new(body);
    draft.add_media(handle.id);
    let post = draft.send(&token).await?;
    let user = post.response.user.unwrap();
    println!("Successfully tweeted:");
    println!("@{} `{}`: `{}` (image: `{}`)", &user.screen_name, &user.name, &post.response.text, &image_fname);
    Ok(())
}

#[allow(dead_code)]
pub fn tweet_w_img_await(body: String, img_path: String, token: &Token) -> Result<()> {
    let res = rt().block_on(async {
        // tweet("test2".to_string(), &token).await.unwrap();
        let res = tweet_w_img(body, img_path, &token).await;
        let res = match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
        res
    });
    res
}

#[allow(dead_code)]
fn main() {
    let token = create_token("config/config.yml".to_string());
    rt().block_on(async {
        tweet("テストツイート（bot のメンテナンス中です）".to_string(), &token).await.unwrap();
        // tweet_w_img("test w/img".to_string(), "assets/test/test.png".to_string(), &token).await.unwrap();
    });
}
