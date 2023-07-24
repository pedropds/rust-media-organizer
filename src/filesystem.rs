use std::fs;

pub fn move_file(path: &str, new_parent_folder: &str, file_name: &str) {
    if new_parent_folder.is_empty() { return; }

    let new_path = format!("{}/{}", new_parent_folder, file_name);

    let op = fs::rename(path, new_path);
    match op {
        Ok(_) => println!("Moved file {} to {}", file_name, new_parent_folder),
        Err(_) => println!("Error moving file {} to {}", file_name, new_parent_folder)
    }
}