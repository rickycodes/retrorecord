use crate::utils::read_env_var;
use egg_mode::media::media_types::image_png;
use egg_mode::media::UploadBuilder;
use egg_mode::tweet::DraftTweet;
use egg_mode::{verify_tokens, KeyPair, Token};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use tokio_core::reactor::Core;

pub fn get_token() -> Token {
    let con_token = KeyPair::new(
        read_env_var("TWITTER_CONSUMER_KEY"),
        read_env_var("TWITTER_CONSUMER_SECRET"),
    );

    let acc_token = KeyPair::new(
        read_env_var("TWITTER_ACCESS_TOKEN"),
        read_env_var("TWITTER_ACCESS_TOKEN_SECRET"),
    );

    Token::Access {
        consumer: con_token,
        access: acc_token,
    }
}

pub fn tweet(message: String, shots: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut core = Core::new().unwrap();

    let token = get_token();

    if let Err(error) = core.run(verify_tokens(&token)) {
        println!("there is something wrong with the tokens: {:?}", error);
    }

    let mut media_ids = Vec::new();

    for media in shots {
        let mut buffer = Vec::new();
        {
            let mut file = File::open(media.clone()).expect("cannot open picture file..");
            let _ = file
                .read_to_end(&mut buffer)
                .expect("cannot read picture file..");
        }

        let upload_builder = UploadBuilder::new(buffer, image_png());
        let media_handler = core
            .run(upload_builder.call(&token))
            .expect("handling media failed..");

        media_ids.push(media_handler.id)
    }

    let tweet = DraftTweet::new(message).media_ids(&media_ids);

    core.run(tweet.send(&token)).expect("tweet failed..");

    Ok(())
}
