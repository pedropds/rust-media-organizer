use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, Result, INotifyWatcher, Event};
use crate::{
    AUDIO_FILE_EXTENSIONS,
    AUDIO_FOLDER_PATH,
    DOCUMENT_FILE_EXTENSIONS,
    DOCUMENT_FOLDER_PATH,
    IMAGE_FILE_EXTENSIONS,
    IMAGE_FOLDER_PATH,
    VIDEO_FILE_EXTENSIONS,
    VIDEO_FOLDER_PATH,
    WATCH_FOLDER_PATH,
    filesystem::move_file,
};

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

    if !unwrapped_event.kind.is_create() { return; }

    let paths = unwrapped_event.paths;

    let image_file_extensions: Vec<&str> = IMAGE_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let video_file_extensions: Vec<&str> = VIDEO_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let audio_file_extensions: Vec<&str> = AUDIO_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let document_file_extensions: Vec<&str> = DOCUMENT_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();

    paths.iter()
        .filter(|path| path.is_file())
        .for_each(|path| {
            let file_type = path.extension().unwrap().to_str().unwrap();
            let file_path = path.to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            match file_type {
                file_type if image_file_extensions.contains(&file_type) => move_file(file_path, IMAGE_FOLDER_PATH, file_name),
                file_type if video_file_extensions.contains(&file_type) => move_file(file_path, VIDEO_FOLDER_PATH, file_name),
                file_type if audio_file_extensions.contains(&file_type) => move_file(file_path, AUDIO_FOLDER_PATH, file_name),
                file_type if document_file_extensions.contains(&file_type) => move_file(file_path, DOCUMENT_FOLDER_PATH, file_name),
                _ => {}
            }
        });
}