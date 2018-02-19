use notify::{RecommendedWatcher, Watcher, raw_watcher, RecursiveMode, op, RawEvent};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::spawn;
use std::path::PathBuf;

pub struct FileWatcher {
  pub watcher : RecommendedWatcher,
  pub change_events: Receiver<RawEvent>,
}

pub fn watch(path: &str) -> FileWatcher {
  let (tx, rx) = channel::<RawEvent>();
  let mut watcher = raw_watcher(tx).unwrap();
  watcher.watch(path, RecursiveMode::Recursive).unwrap();

  FileWatcher {
    watcher: watcher,
    change_events: rx,
  }
}

pub fn spawn_watcher(path: &str, f: fn(PathBuf)) -> thread::JoinHandle<()> {
  let watcher = watch(&path);

  spawn(move|| {
    loop {
      match watcher.change_events.recv() {
        Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
          // println!("op is: {:?}", op);
          if op == op::CLOSE_WRITE {
            f(path);
          }
        },
        Ok(event) => println!("broken event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
  })
}
