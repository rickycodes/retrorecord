extern crate egg_mode;
extern crate tokio_core;
extern crate notify;
extern crate mime;
extern crate rand;
extern crate dialoguer;
extern crate regex;
extern crate clap;

mod watch;
mod tweet;
mod message;
mod bots;
mod utils;
mod screenshot;
mod recording;
mod ask;
mod test_path;

use clap::{Arg, App};
use screenshot::screenshot;
use recording::recording;
use watch::spawn_watcher;
use utils::read_env_var;

fn main() {
  let matches = App::new("retrorecord")
    .version("1.0")
    .author("Ricky Miller <ricky.miller@gmail.com>")
    .about("post screenshots/recrord video games from RetroPie to twitter")
    .arg(
      Arg::with_name("prompt")
        .short("p")
        .required(false)
        .long("prompt")
        .help("prompt to post")
        .takes_value(false)
    )
    .get_matches();

  let prompt: bool = matches.is_present("prompt");

  println!("application started...\nprompts are {}", if prompt { "ON" } else { "OFF" });

  let screenshots_thread = spawn_watcher(
    &read_env_var("SCREENSHOTS_DIR"),
    screenshot,
    prompt
  );
  let recordings_thread = spawn_watcher(
    &read_env_var("RECORDINGS_DIR"),
    recording,
    prompt
  );
  screenshots_thread
    .join()
    .expect("The screenshots thread has panicked");
  recordings_thread
    .join()
    .expect("The recordings thread has panicked");
}
