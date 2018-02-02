mod config;

use std::error::Error;
use egg_mode::{self, KeyPair, Token};
use tokio_core::reactor::{Core};

pub fn tweet(message: &str) -> Result<(), Box<Error>> {
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

    let draft = egg_mode::tweet::DraftTweet::new(message);
    core.run(draft.send(&token, &handle))?;

    Ok(())
}
