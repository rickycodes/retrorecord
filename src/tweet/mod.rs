mod config;

extern crate mime;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token};
use egg_mode::media::UploadBuilder;
use self::mime::Mime;
use tokio_core::reactor::{Core};

pub fn get_token() -> Token {
    let con_token = KeyPair::new(
        config::TWITTER_CONSUMER_KEY,
        config::TWITTER_CONSUMER_SECRET
    );

    let acc_token = KeyPair::new(
        config::TWITTER_ACCESS_TOKEN,
        config::TWITTER_ACCESS_TOKEN_SECRET
    );

    let token = Token::Access {
        consumer: con_token,
        access: acc_token,
    };

    token
}

pub fn tweet(message: String, buffer: &'static [u8], media_type: Mime) -> Result<(), Box<Error>> {
    println!("trying to do a tweet");
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let token = get_token();

    /* let mut buffer = Vec::new();

    {
        let mut file = File::open(media.clone()).expect("cannot open picture file..");
        let _ = file.read_to_end(&mut buffer).expect("cannot read picture file..");
    } */

    println!("buffer created!");

    let upload_builder = UploadBuilder::new(buffer, media_type);
    let media_handler = core.run(upload_builder.call(&token, &handle)).expect("handling media failed..");

    println!("trying to do a draft!");

    let tweet = DraftTweet::new(message).media_ids(&[media_handler.id]);

    println!("draft created!");

    core.run(tweet.send(&token, &handle)).expect("tweet failed..");

    Ok(())
}
