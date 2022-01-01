extern crate dialoguer;
extern crate egg_mode;
extern crate mime;
extern crate notify;
extern crate regex;

mod constants;
mod tweet;
mod utils;

use utils::read_env_var;

use crate::utils::{path_to_string, prep_tweet, watch};
use notify::{op, RawEvent};

use crate::constants::{
    EMPTY, GROUP_BY, NOT_SET, SCREENSHOTS_DIR
};

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
