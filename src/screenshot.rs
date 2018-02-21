use egg_mode::media::media_types::image_png;
use tweet::tweet;
use message::get_message;
use utils::path_to_string;
use bots::get_bots;
use std::path::PathBuf;
use ask::ask;

pub fn screenshot(path: PathBuf) {
  println!("file written: {:?}", path);

  if ask("Would you like to post this screenshot?") {
    if let Ok(_) = tweet(get_message(get_bots(3)), path_to_string(path), image_png()) {
      println!("posted tweet!");
    }
  }
}
