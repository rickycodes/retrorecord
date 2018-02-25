use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token, verify_tokens};
use egg_mode::media::UploadBuilder;
use mime::Mime;
use tokio_core::reactor::{Core};
use config::{
  TWITTER_CONSUMER_KEY,
  TWITTER_CONSUMER_SECRET,
  TWITTER_ACCESS_TOKEN,
  TWITTER_ACCESS_TOKEN_SECRET
};

pub fn get_token() -> Token {
  let con_token = KeyPair::new(
    TWITTER_CONSUMER_KEY,
    TWITTER_CONSUMER_SECRET
  );

  let acc_token = KeyPair::new(
    TWITTER_ACCESS_TOKEN,
    TWITTER_ACCESS_TOKEN_SECRET
  );

  Token::Access {
    consumer: con_token,
    access: acc_token,
  }
}

pub fn tweet(message: String, media: String, media_type: Mime) -> Result<(), Box<Error>> {
  let mut core = Core::new().unwrap();
  let handle = core.handle();

  let token = get_token();

  if let Err(error) = core.run(verify_tokens(&token, &handle)) {
    println!("there is something wrong with the tokens: {:?}", error);
  }

  let mut buffer = Vec::new();

  {
    let mut file = File::open(media.clone()).expect("cannot open picture file..");
    let _ = file.read_to_end(&mut buffer).expect("cannot read picture file..");
  }

  let upload_builder = UploadBuilder::new(buffer, media_type);
  let media_handler = core.run(upload_builder.call(&token, &handle)).expect("handling media failed..");

  let tweet = DraftTweet::new(message).media_ids(&[media_handler.id]);

  core.run(tweet.send(&token, &handle)).expect("tweet failed..");

  Ok(())
}
