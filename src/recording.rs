use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use egg_mode::media::media_types::image_gif;
use tweet::tweet;
use message::get_message;
use utils::path_to_string;
use ask::ask;
use config::GIFS_DIR;
use test_path::test_path;

pub fn recording(path_string: String) {
  let copy = path_string.clone();

  if test_path(path_string, r"\.mkv") {
    println!("file written: {:?}", copy);

    if ask("Would you like to post this recording?") {
      let gifs = GIFS_DIR;
      let current_dir = env::current_dir().unwrap();

      let sh = Path::new(&current_dir).join("mkvToGif.sh");

      let child = Command::new("bash")
        .args(&[path_to_string(sh), copy, gifs.to_string()])
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
    } else {
      // trash the file
      Command::new("rm")
        .arg(&copy)
        .spawn()
        .expect("failed remove file");
    }
  }
}
