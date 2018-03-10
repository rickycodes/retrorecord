use egg_mode::media::media_types::image_png;
use tweet::tweet;
use message::get_message;
use bots::get_bots;
use ask::ask;
use test_path::test_path;

pub fn screenshot(path_string: String, prompt: bool) {
  let copy = path_string.clone();

  if test_path(path_string, r"\.png") {
    println!("file written: {:?}", copy);

    if ask("Would you like to post this screenshot?", prompt) {
      if let Ok(_) = tweet(
        get_message(get_bots(3)),
        copy,
        image_png()
      ) {
        println!("posted tweet!");
      }
    }
  }
}
