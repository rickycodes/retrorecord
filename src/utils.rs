use crate::constants::{
    TWITTER_ACCESS_TOKEN, TWITTER_ACCESS_TOKEN_SECRET, TWITTER_CONSUMER_KEY,
    TWITTER_CONSUMER_SECRET,
};

use egg_mode::{KeyPair, Token};

use std::env;
use std::path::PathBuf;

use crate::tweet::tweet;
use dialoguer::{theme::ColorfulTheme, Input};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use crate::constants::{
    FileWatcher, INITIAL_PROMPT, POSTED_TWEET, SPACE, TAGS
};

pub fn path_to_string(path: PathBuf) -> String {
    path.into_os_string().into_string().unwrap()
}

pub fn read_env_var(env_var: &str) -> String {
    return match env::var(env_var) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Couldn't read {} ({})", env_var, e);
            ::std::process::exit(1);
        }
    };
}

pub fn get_token() -> Token {
    let con_token = KeyPair::new(
        read_env_var(TWITTER_CONSUMER_KEY),
        read_env_var(TWITTER_CONSUMER_SECRET),
    );

    let acc_token = KeyPair::new(
        read_env_var(TWITTER_ACCESS_TOKEN),
        read_env_var(TWITTER_ACCESS_TOKEN_SECRET),
    );

    Token::Access {
        consumer: con_token,
        access: acc_token,
    }
}

pub fn watch(path: &str) -> FileWatcher {
    let (tx, rx) = channel::<RawEvent>();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    FileWatcher {
        watcher,
        change_events: rx,
    }
}

fn get_input() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(INITIAL_PROMPT)
        .interact_text()
        .unwrap()
}

pub fn prep_tweet(shots: Vec<String>) {
    let input = get_input();
    let content = input + SPACE + &TAGS.join(SPACE);

    println!("{:?}", content);

    if tweet(content, shots).is_ok() {
        println!("{:?}", POSTED_TWEET);
    }
}
