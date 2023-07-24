mod file_watcher;
mod filesystem;

#[macro_use]
extern crate dotenv_codegen;

use notify::{Result};

//global variables
const WATCH_FOLDER_PATH: &str = dotenv!("WATCH_FOLDER_PATH");
const IMAGE_FILE_EXTENSIONS: &str = dotenv!("IMAGE_FILE_EXTENSIONS");
const VIDEO_FILE_EXTENSIONS: &str = dotenv!("VIDEO_FILE_EXTENSIONS");
const AUDIO_FILE_EXTENSIONS: &str = dotenv!("AUDIO_FILE_EXTENSIONS");
const DOCUMENT_FILE_EXTENSIONS: &str = dotenv!("DOCUMENT_FILE_EXTENSIONS");

const IMAGE_FOLDER_PATH: &str = dotenv!("IMAGE_FOLDER_PATH");
const VIDEO_FOLDER_PATH: &str = dotenv!("VIDEO_FOLDER_PATH");
const AUDIO_FOLDER_PATH: &str = dotenv!("AUDIO_FOLDER_PATH");
const DOCUMENT_FOLDER_PATH: &str = dotenv!("DOCUMENT_FOLDER_PATH");

fn main() -> Result<()> {
    verify_env_variables();
    println!("Watching folder: {}", WATCH_FOLDER_PATH);
    file_watcher::watch()
}

fn verify_env_variables() {
    if WATCH_FOLDER_PATH.is_empty() {
        panic!("WATCH_FOLDER_PATH is not set in .env file");
    }

    if !IMAGE_FILE_EXTENSIONS.is_empty() && IMAGE_FOLDER_PATH.is_empty() {
        println!("IMAGE_FOLDER_PATH is not set in .env file. This is required for image files to be moved");
    }

    if !VIDEO_FILE_EXTENSIONS.is_empty() && VIDEO_FOLDER_PATH.is_empty() {
        println!("VIDEO_FOLDER_PATH is not set in .env file. This is required for video files to be moved");
    }

    if !AUDIO_FILE_EXTENSIONS.is_empty() && AUDIO_FOLDER_PATH.is_empty() {
        println!("AUDIO_FOLDER_PATH is not set in .env file. This is required for audio files to be moved");
    }

    if !DOCUMENT_FILE_EXTENSIONS.is_empty() && DOCUMENT_FOLDER_PATH.is_empty() {
        println!("DOCUMENT_FOLDER_PATH is not set in .env file. This is required for document files to be moved");
    }
}
