mod config;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token};
use egg_mode::media::{UploadBuilder, media_types};
use tokio_core::reactor::{Core};


pub fn tweet(message: &str, media_path: String) -> Result<(), Box<Error>> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

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

    let mut buffer = Vec::new();

    {
        let mut file = File::open(media_path.clone()).expect("cannot open picture file..");
        let _ = file.read_to_end(&mut buffer).expect("cannot read picture file..");
    }

    let upload_builder = UploadBuilder::new(buffer, media_types::image_png());
    let media_handler = core.run(upload_builder.call(&token,&handle)).expect("handling media failed..");

    let tweet_draft = DraftTweet::new(message).media_ids(&[media_handler.id]);

    core.run(tweet_draft.send(&token, &handle)).expect("tweet failed..");

    Ok(())
}
