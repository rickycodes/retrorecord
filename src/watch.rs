extern crate notify;

use self::notify::{op, Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use std::path::PathBuf;

pub fn watch(path: &str) -> PathBuf {
    let (tx, rx) = channel();

    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                // println!("op is: {:?}", op);
                if op == op::CLOSE_WRITE {
                    return path
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
