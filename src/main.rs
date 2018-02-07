extern crate egg_mode;
extern crate tokio_core;
extern crate notify;

use std::process::{Command, Stdio};
use notify::{op, RawEvent};
use std::{thread, env};
use std::path::{Path, PathBuf};
use egg_mode::media::media_types;

mod watch;
mod tweet;
mod message;
mod bots;

fn path_to_string(path: PathBuf) -> String {
    path.into_os_string()
        .into_string()
        .unwrap()
}

fn main() {
    let screenshots = "/home/pi/.config/retroarch/screenshots/";
    let recordings = "/home/pi/recordings/";

    let screenshots_watcher = watch::watch(&screenshots);
    let recordings_watcher = watch::watch(&recordings);

    let screenshots_handler = thread::spawn(move|| {
        loop {
            match screenshots_watcher.change_events.recv() {
                Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                    // println!("op is: {:?}", op);
                    if op == op::CLOSE_WRITE {
                        println!("file written: {:?}", path);
                        let screenshots_path = path_to_string(path);

                        if let Ok(_) = tweet::tweet(message::get_message(bots::get_bots()), screenshots_path, media_types::image_png()) {
                            println!("posted tweet!");
                        }
                    }
                },
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    let recordings_handler = thread::spawn(move|| {
        loop {
            match recordings_watcher.change_events.recv() {
                Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                    println!("op is: {:?}, path is: {:?}", op, path);
                    if op == op::CLOSE_WRITE {
                        println!("file written: {:?}", path);
                        let recordings_path = path_to_string(path);

                        let gifs = "/home/pi/gifs/";
                        let current_dir = env::current_dir().unwrap();

                        println!("current_dir is: {:?}", current_dir);
                        let sh = Path::new(&current_dir).join("mkvToGif.sh");
                        println!("sh is: {:?}", sh);

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
                            let gif = path_to_string(output);
                            println!("gif is {:?}", gif);
                            if let Ok(_) = tweet::tweet(message::get_message("@DATAM0SHER".to_string()), gif, media_types::image_gif()) {
                                println!("posted tweet!");
                            }
                        }
                    }
                },
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    screenshots_handler.join().expect("The screenshots thread has panicked");
    recordings_handler.join().expect("The recordings thread has panicked");
}
