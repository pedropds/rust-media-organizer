use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, Result, INotifyWatcher, Event};
use crate::{WATCH_FOLDER_PATH};
use crate::filesystem::process_paths;

pub fn watch() -> Result<()> {
    let (sender, receiver) = channel();

    let config = notify::Config::default()
        .with_poll_interval(Duration::from_secs(5))
        .with_compare_contents(true);

    let mut watcher: INotifyWatcher = Watcher::new(sender, config)?;
    watcher.watch(Path::new(WATCH_FOLDER_PATH), RecursiveMode::Recursive)?;

    loop {
        match receiver.recv() {
            Ok(event) => handle_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_event(event: Result<Event>) {
    let unwrapped_event = event.unwrap();
    println!("Event: {:?}", unwrapped_event);

    if !unwrapped_event.kind.is_create() {
        return;
    }

    process_paths(unwrapped_event.paths.into_iter());
}