use std::fs;
use std::path::PathBuf;
use crate::{AUDIO_FILE_EXTENSIONS, AUDIO_FOLDER_PATH, DOCUMENT_FILE_EXTENSIONS, DOCUMENT_FOLDER_PATH, IMAGE_FILE_EXTENSIONS, IMAGE_FOLDER_PATH, VIDEO_FILE_EXTENSIONS, VIDEO_FOLDER_PATH, WATCH_FOLDER_PATH};

pub fn move_file(path: &str, new_parent_folder: &str, file_name: &str) {
    if new_parent_folder.is_empty() { return; }

    let new_path = format!("{}/{}", new_parent_folder, file_name);

    let op = fs::rename(path, new_path);
    match op {
        Ok(_) => println!("Moved file {} to {}", file_name, new_parent_folder),
        Err(_) => println!("Error moving file {} to {}", file_name, new_parent_folder)
    }
}

pub fn organize_download_folder() {
    let paths = fs::read_dir(WATCH_FOLDER_PATH).unwrap();
    process_paths(paths.filter_map(|path| path.ok()).map(|path| path.path()));
}

pub fn process_paths(paths: impl Iterator<Item = PathBuf>) {
    let image_file_extensions: Vec<&str> = IMAGE_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let video_file_extensions: Vec<&str> = VIDEO_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let audio_file_extensions: Vec<&str> = AUDIO_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();
    let document_file_extensions: Vec<&str> = DOCUMENT_FILE_EXTENSIONS.split(",").map(|str: &str| str.trim()).collect();

    paths
        .filter(|path| path.is_file())
        .for_each(|path| {
            let file_type = path.extension().unwrap().to_str().unwrap();
            let file_path = path.to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            match file_type {
                file_type if image_file_extensions.contains(&file_type)
                    && !IMAGE_FOLDER_PATH.is_empty() => move_file(file_path, IMAGE_FOLDER_PATH, file_name),
                file_type if video_file_extensions.contains(&file_type)
                    && !VIDEO_FOLDER_PATH.is_empty() => move_file(file_path, VIDEO_FOLDER_PATH, file_name),
                file_type if audio_file_extensions.contains(&file_type)
                    &&  !AUDIO_FOLDER_PATH.is_empty() => move_file(file_path, AUDIO_FOLDER_PATH, file_name),
                file_type if document_file_extensions.contains(&file_type)
                    && !DOCUMENT_FOLDER_PATH.is_empty() => move_file(file_path, DOCUMENT_FOLDER_PATH, file_name),
                _ => {}
            }
        });
}