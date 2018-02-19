use egg_mode::media::media_types::image_png;

use tweet::tweet;
use message::get_message;
use utils::path_to_string;
use bots::get_bots;
use std::path::PathBuf;

pub fn screenshot(path: PathBuf) {
  println!("file written: {:?}", path);

  if let Ok(_) = tweet(get_message(get_bots(3)), path_to_string(path), image_png()) {
    println!("posted tweet!");
  }
}
