extern crate egg_mode;
extern crate tokio_core;
extern crate notify;

use std::process::{Command, Stdio};
use std::path::PathBuf;
use notify::{op, RawEvent};

mod watch;
mod tweet;
mod message;

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    let recordings = "/home/pi/recordings/";

    let screenshots_watcher = watch::watch(&screenshots);
    let recordings_watcher = watch::watch(&recordings);

    loop {
        match screenshots_watcher.change_events.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                // println!("op is: {:?}", op);
                if op == op::CLOSE_WRITE {
                    println!("file written: {:?}", path);
                    let screenshots_path = path
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    if let Ok(_) = tweet::tweet(message::get_message(), screenshots_path) {
                        println!("posted tweet!");
                    }
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }

        /* match recordings_watcher.change_events.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                println!("op is: {:?}, path is: {:?}", op, path);
                if op == op::CLOSE_WRITE {
                    println!("file written: {:?}", path);
                    let recordings_path = path_to_string(path);

                    let gifs = "/home/pi/gifs/";
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
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        } */
    }
}
