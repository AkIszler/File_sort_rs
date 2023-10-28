use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    // Get the current working directory.
    let current_directory = std::env::current_dir().expect("Failed to get current directory");
    
    // Prompt the user for the file extension.
    println!("Enter the file extension (e.g., 'png'): ");
    let mut file_extension = String::new();
    io::stdin()
        .read_line(&mut file_extension)
        .expect("Failed to read input");
    file_extension = file_extension.trim().to_lowercase();

    // Create a subfolder with the name of the file extension.
    let subfolder_name = format!("./{}_files", file_extension);

    // Combine the current directory with the subfolder name.
    let target_directory = current_directory.join(&subfolder_name);

    if !target_directory.exists() {
        fs::create_dir(&target_directory).expect("Failed to create subfolder");
    }
    // List files in the current directory.
    if let Ok(entries) = fs::read_dir(&current_directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path: PathBuf = entry.path();

                // Check if the file has the specified file extension.
                if let Some(extension) = file_path.extension() {
                    if extension.to_string_lossy().to_lowercase() == file_extension {
                        // Move the file to the subfolder.
                        let path_join = file_path.file_name().unwrap();
                        let new_path = target_directory.join(path_join);
                        fs::rename(&file_path, &new_path).expect("Failed to move file");
                    }
                }
            }
        }
    }
}
