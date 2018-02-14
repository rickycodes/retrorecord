use notify::{op, RawEvent};
use std::thread;
use std::thread::spawn;
use egg_mode::media::media_types::image_png;

use tweet::tweet;
use message::get_message;
use utils::path_to_string;
use watch::watch;
use bots::get_bots;

pub fn screenshots() -> thread::JoinHandle<()> {
  let screenshots = "/home/pi/.config/retroarch/screenshots/";
  let screenshots_watcher = watch(&screenshots);

  spawn(move|| {
    loop {
      match screenshots_watcher.change_events.recv() {
        Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
          // println!("op is: {:?}", op);
          if op == op::CLOSE_WRITE {
            println!("file written: {:?}", path);
            let screenshots_path = path_to_string(path);

            if let Ok(_) = tweet(get_message(get_bots()), screenshots_path, image_png()) {
              println!("posted tweet!");
            }
          }
        },
        Ok(event) => println!("broken event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
  })
}
