extern crate dialoguer;
extern crate egg_mode;
extern crate mime;
extern crate notify;
extern crate regex;

mod constants;
mod tweet;
mod utils;

use utils::read_env_var;

use crate::tweet::tweet;
use crate::utils::path_to_string;
use dialoguer::{theme::ColorfulTheme, Input};
use notify::{op, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use crate::constants::{
    GROUP_BY, FileWatcher, EMPTY, INITIAL_PROMPT, NOT_SET, POSTED_TWEET, SCREENSHOTS_DIR, SPACE, TAGS,
};

fn watch(path: &str) -> FileWatcher {
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

fn prep_tweet(shots: Vec<String>) {
    let input = get_input();
    let content = input + SPACE + &TAGS.join(SPACE);

    println!("{:?}", content);

    if tweet(content, shots).is_ok() {
        println!("{:?}", POSTED_TWEET);
    }
}

fn main() {
    if SCREENSHOTS_DIR == EMPTY {
        panic!("{:?}", SCREENSHOTS_DIR.to_owned() + NOT_SET);
    }
    let path = &read_env_var(SCREENSHOTS_DIR);
    let watcher = watch(path);
    let mut shots = Vec::new();

    loop {
        match watcher.change_events.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                ..
            }) => {
                if op == op::CLOSE_WRITE {
                    shots.push(path_to_string(path));

                    println!("count: {}", shots.len());

                    if shots.len() % GROUP_BY == 0 {
                        prep_tweet(shots.clone());
                    }
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(event) => println!("watch error: {:?}", event),
        }
    }
}
