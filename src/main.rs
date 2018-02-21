extern crate egg_mode;
extern crate tokio_core;
extern crate notify;
extern crate mime;
extern crate rand;
extern crate dialoguer;

mod watch;
mod tweet;
mod message;
mod bots;
mod utils;
mod screenshot;
mod recording;
mod ask;

use screenshot::screenshot;
use recording::recording;
use watch::spawn_watcher;

fn main() {
  println!("application started...");
  let screenshots_thread = spawn_watcher(
    "/home/pi/.config/retroarch/screenshots/",
    screenshot
  );
  let recordings_thread = spawn_watcher(
    "/home/pi/recordings/",
    recording
  );
  screenshots_thread.join().expect("The screenshots thread has panicked");
  recordings_thread.join().expect("The recordings thread has panicked");
}
