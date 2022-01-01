use crate::constants::{CANNOT_OPEN, CANNOT_READ, GROUP_BY, HANDLING_FAILED, TWEET_FAILED};
use crate::utils::get_token;
use egg_mode::media::media_types::image_png;
use egg_mode::media::UploadBuilder;
use egg_mode::tweet::DraftTweet;
use egg_mode::verify_tokens;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use tokio_core::reactor::Core;

pub fn tweet(message: String, shots: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut core = Core::new().unwrap();

    let token = get_token();

    if let Err(error) = core.run(verify_tokens(&token)) {
        println!("there is something wrong with the tokens: {:?}", error);
    }

    let mut media_ids = Vec::new();
    let last_four = &shots[shots.len() - GROUP_BY..];

    for media in last_four {
        let mut buffer = Vec::new();
        {
            let mut file = File::open(media.clone()).expect(CANNOT_OPEN);
            let _ = file.read_to_end(&mut buffer).expect(CANNOT_READ);
        }

        let upload_builder = UploadBuilder::new(buffer, image_png());
        let media_handler = core
            .run(upload_builder.call(&token))
            .expect(HANDLING_FAILED);

        media_ids.push(media_handler.id)
    }

    let tweet = DraftTweet::new(message).media_ids(&media_ids);

    println!("{:?}", tweet);

    core.run(tweet.send(&token)).expect(TWEET_FAILED);

    Ok(())
}
