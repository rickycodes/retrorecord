extern crate egg_mode;
extern crate tokio_core;
extern crate notify;
extern crate mime;
extern crate rand;
extern crate dialoguer;
extern crate regex;

mod watch;
mod tweet;
mod message;
mod bots;
mod utils;
mod screenshot;
mod recording;
mod ask;
mod config;
mod test_path;

use screenshot::screenshot;
use recording::recording;
use watch::spawn_watcher;
use config::{
  SCREENSHOTS_DIR,
  RECORDINGS_DIR
};

fn main() {
  println!("application started...");
  let screenshots_thread = spawn_watcher(
    SCREENSHOTS_DIR,
    screenshot
  );
  let recordings_thread = spawn_watcher(
    RECORDINGS_DIR,
    recording
  );
  screenshots_thread
    .join()
    .expect("The screenshots thread has panicked");
  recordings_thread
    .join()
    .expect("The recordings thread has panicked");
}
