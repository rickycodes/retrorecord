use notify::{RecommendedWatcher, Watcher, raw_watcher, RecursiveMode, RawEvent};
use std::sync::mpsc::{channel, Receiver};

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
