extern crate egg_mode;
extern crate tokio_core;
extern crate notify;
extern crate mime;
extern crate rand;

mod watch;
mod tweet;
mod message;
mod bots;
mod utils;
mod screenshots;
mod recordings;

use screenshots::screenshots;
use recordings::recordings;

fn main() {
  println!("application started...");
  let screenshots_thread = screenshots();
  let recordings_thread = recordings();
  screenshots_thread.join().expect("The screenshots thread has panicked");
  recordings_thread.join().expect("The recordings thread has panicked");
}
