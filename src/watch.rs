use notify::{RecommendedWatcher, Watcher, raw_watcher, RecursiveMode, op, RawEvent};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::spawn;
use utils::path_to_string;

struct FileWatcher {
  pub watcher : RecommendedWatcher,
  pub change_events: Receiver<RawEvent>,
}

fn watch(path: &str) -> FileWatcher {
  let (tx, rx) = channel::<RawEvent>();
  let mut watcher = raw_watcher(tx).unwrap();
  watcher.watch(path, RecursiveMode::Recursive).unwrap();

  FileWatcher {
    watcher: watcher,
    change_events: rx,
  }
}

pub fn spawn_watcher(path: &str, f: fn(String, bool), prompt: bool) -> thread::JoinHandle<()> {
  let watcher = watch(path);

  spawn(move|| {
    loop {
      match watcher.change_events.recv() {
        Ok(RawEvent{path: Some(path), op: Ok(op), ..}) => {
          // println!("op is: {:?}", op);
          if op == op::CLOSE_WRITE {
            f(path_to_string(path), prompt);
          }
        },
        Ok(event) => println!("broken event: {:?}", event),
        Err(event) => println!("watch error: {:?}", event),
      }
    }
  })
}

#[cfg(test)]
mod test {
  use std::process::Command;
  use std::env;
  use std::path::Path;
  use utils::path_to_string;
  use super::*;

  #[test]
  fn spawn_watcher_close_write_op() {
    let current_dir = env::current_dir().unwrap();
    let test_file = Path::new(&current_dir).join("test.txt");

    fn f(_path: String) {
      assert!(true);
    }

    spawn_watcher(
      &path_to_string(current_dir),
      f
    );

    Command::new("touch")
      .arg(&path_to_string(test_file))
      .spawn()
      .expect("failed to execute child");
  }
}
