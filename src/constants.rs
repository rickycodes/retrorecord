use notify::{RawEvent, RecommendedWatcher};
use std::sync::mpsc::Receiver;

pub const GROUP_BY: usize = 4;

// could also have tag input?
pub const TAGS: [&str; 1] = ["#MiSTerFPGA"];

// could also do something with @?

pub const SPACE: &str = " ";
pub const EMPTY: &str = "";
pub const INITIAL_PROMPT: &str = "tweet content:";
pub const SCREENSHOTS_DIR: &str = "SCREENSHOTS_DIR";
pub const TWITTER_CONSUMER_KEY: &str = "TWITTER_CONSUMER_KEY";
pub const TWITTER_CONSUMER_SECRET: &str = "TWITTER_CONSUMER_SECRET";
pub const TWITTER_ACCESS_TOKEN: &str = "TWITTER_ACCESS_TOKEN";
pub const TWITTER_ACCESS_TOKEN_SECRET: &str = "TWITTER_ACCESS_TOKEN_SECRET";
pub const NOT_SET: &str = " NOT SET!";

pub const CANNOT_OPEN: &str = "cannot open picture file..";
pub const CANNOT_READ: &str = "cannot read picture file..";
pub const HANDLING_FAILED: &str = "handling media failed..";
pub const TWEET_FAILED: &str = "tweet failed..";
pub const POSTED_TWEET: &str = "posted tweet!";

pub struct FileWatcher {
    #[allow(dead_code)]
    pub watcher: RecommendedWatcher,
    pub change_events: Receiver<RawEvent>,
}
