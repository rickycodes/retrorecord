extern crate egg_mode;
extern crate tokio_core;

use std::process::{Command, Stdio};

mod watch;
mod tweet;
mod message;

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    let recordings = "/home/pi/recordings/";

    loop {
        let screenshots_path = watch::watch(screenshots);
        if let Ok(_) = tweet::tweet(message::get_message(), screenshots_path) {
            println!("posted tweet!");
        }

        let recordings_path = watch::watch(recordings);
        let gifs = "/home/pi/gifs/";
        println!("{:?}", recordings_path);

        let child = Command::new("mkvToGif.sh")
            .args(&[recordings_path, gifs.to_string()])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let output = child
            .wait_with_output()
            .expect("failed to wait on child");

        if output.status.success() {
            println!("gif complete!");
        }
    }
}
