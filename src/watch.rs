extern crate notify;

use self::notify::{op, Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

pub fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch("/home/pi/.config/retroarch/screenshots/", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                if op == op::CLOSE_WRITE {
                    println!("Screenshot taken: {:?}", path)
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
