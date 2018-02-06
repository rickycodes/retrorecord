extern crate notify;

use self::notify::{op, Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

pub fn watch(path: &str) -> String {
    let (tx, rx) = channel();

    let mut watcher = raw_watcher(tx).unwrap();

    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
                // println!("op is: {:?}", op);
                if op == op::CLOSE_WRITE {
                    println!("file written: {:?}", path);
                    return path
                        .into_os_string()
                        .into_string()
                        .unwrap()
                }
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
