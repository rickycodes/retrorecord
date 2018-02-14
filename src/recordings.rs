use std::process::{Command, Stdio};
use notify::{op, RawEvent};
use std::{thread, env};
use std::thread::spawn;
use std::path::Path;
use egg_mode::media::media_types::image_gif;

use tweet::tweet;
use message::get_message;
use utils::path_to_string;
use watch::watch;

pub fn recordings() -> thread::JoinHandle<()> {
  let recordings = "/home/pi/recordings/";
  let recordings_watcher = watch(&recordings);

  spawn(move|| {
    loop {
      match recordings_watcher.change_events.recv() {
        Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
          // println!("op is: {:?}, path is: {:?}", op, path);
          if op == op::CLOSE_WRITE {
            println!("file written: {:?}", path);
            let recordings_path = path_to_string(path);

            let gifs = "/home/pi/gifs/";
            let current_dir = env::current_dir().unwrap();

            let sh = Path::new(&current_dir).join("mkvToGif.sh");

            let child = Command::new("bash")
              .args(&[path_to_string(sh), recordings_path, gifs.to_string()])
              .stdout(Stdio::piped())
              .spawn()
              .expect("failed to execute child");

            let output = child
              .wait_with_output()
              .expect("failed to wait on child");

            if output.status.success() {
              println!("gif complete!");
              let output = Path::new(&gifs).join("output.gif");
              let gif_path = path_to_string(output);

              println!("gif_path is {:?}", gif_path);

              if let Ok(_) = tweet(get_message("@DATAM0SHER".to_string()), gif_path, image_gif()) {
                println!("posted tweet!");
              }
            }
          }
        },
        Ok(event) => println!("broken event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
  })
}
