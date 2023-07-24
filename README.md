# MediaOrganizer developed using Rust.
**Description:**

MediaOrganizer is a command-line tool written in Rust that helps users automate the organization of files based on their types into designated folders. The project aims to simplify the process of managing large amounts of files by automatically sorting them into appropriate folders, making it easier to locate and access specific files.

**Key Features:**

1.  **File Type Recognition:** The MediaOrganizer can recognize different file types based on their extensions (e.g., images, videos, audio, documents).

2.  **Customizable Configuration:** Users can customize the types of files they want to organize and specify the corresponding target folders.

3.  **Efficient Sorting:** The tool efficiently scans a specified directory and automatically moves files to their respective folders, keeping the original directory clean and organized.

4.  **Error Handling:** The program includes error handling mechanisms to gracefully handle any issues that may arise during the organization process.

5.  **Environment Configuration:** Users can easily configure the tool using a `.env` file to set the source folder and target folders for different file types.


**Usage:**

1.  Install Rust and set up the project dependencies.

2.  Create a `.env` file and define the source folder and target folders for each file type.

3.  Run the MediaOrganizer from the command line.

4.  The program will scan the source folder, identify files based on their extensions, and move them to the appropriate target folders.


**Potential Extensions:**

1.  Recursive Organization: Implement the option to organize files recursively in subdirectories.

2.  File Duplication Handling: Provide options to handle duplicate files to prevent overwriting important files.

3.  File Exclusion: Allow users to specify certain files or folders to exclude from the organization process.

4.  File Tagging: Add an option to tag files based on metadata or specific criteria.


## Example: Organizing Files in the "Downloads" Folder

**Description:**

The MediaOrganizer is designed to continuously watch the "Downloads" folder for any new files that are added. When a new file is detected, the program automatically moves it to the respective target folder based on its file type, ensuring that the "Downloads" folder remains organized and clutter-free.

**Folder Structure Before Organization:**

```markdown
Downloads/
    ├── image1.jpg
    ├── video1.mp4
    ├── document1.pdf
    ├── audio1.mp3
    └──random_file.txt
```

**Step 1: Configuration (`.env` File)**

Create a `.env` file in the root directory of the MediaOrganizer project. In this file, define the source folder as the "Downloads" folder and specify the target folders for each file type.

```markdown
# .env file  
# Source Folder  
WATCH_FOLDER_PATH=/path/to/Downloads 

# Target Folders  
IMAGE_FOLDER_PATH=/path/to/Images 
VIDEO_FOLDER_PATH=/path/to/Videos 
DOCUMENT_FOLDER_PATH=/path/to/Documents 
AUDIO_FOLDER_PATH=/path/to/Audio

#Target File Types
IMAGE_FILE_EXTENSIONS=jpg,jpeg,png,gif,svg,heic,heif,webp  
VIDEO_FILE_EXTENSIONS=mp4,mov,avi,mkv,webm,flv,wmv,m4v  
AUDIO_FILE_EXTENSIONS=mp3,wav,flac,m4a,aac,ogg,opus,wma  
DOCUMENT_FILE_EXTENSIONS=pdf,doc,docx,xls,xlsx,ppt,pptx,txt
```

**Step 2: Running the MediaOrganizer**

Open the terminal, navigate to the project's root directory, and run the MediaOrganizer tool using `cargo run`.

You can also generate the binary and run the binary instead by using:  `cargo build --release
`.
The binary will be located in `target/release`.

**Step 3: Result After Organization**

After running the MediaOrganizer , the program will continuously watch the "Downloads" folder for any new files added. When a new file is detected, it will be automatically sorted into its respective folder based on its file type.

For example, if you add a new file named "image2.png" to the "Downloads" folder:
```markdown
Downloads/
    ├── image1.jpg
    ├── video1.mp4
    ├── document1.pdf
	├── audio1.mp3
    ├── random_file.txt
    └── image2.png  <- New file added
```
The MediaOrganizer will detect the new file "image2.png," recognize it as an image file, and move it to the "Images" folder:
```markdown
Images/
    ├── ... old files
    └── image2.png
```

This process will be continuous, ensuring that any new files added to the "Downloads" folder are automatically organized into their respective folders based on their file types. This helps maintain an organized and clutter-free "Downloads" folder over time.