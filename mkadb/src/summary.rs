use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;


pub fn create_file_and_save_text<P: AsRef<Path>>(path: P, text: &str) -> io::Result<()> {
    let path_ref = path.as_ref();

    // Create all parent directories if they don't exist
    if let Some(parent) = path_ref.parent() {
        fs::create_dir_all(parent)?;
    }

    // Check if the file exists
    if path_ref.exists() {
        println!("File {:?} exists and will be overwritten.", path_ref);
    } else {
        println!("File {:?} does not exist. It will be created.", path_ref);
    }

    // Open the file in write mode, create it if it doesn't exist, and overwrite if it does
    let mut file = OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it does not exist
        .truncate(true) // Truncate the file to zero length if it exists
        .open(path)?;

    // Write the text to the file
    file.write_all(text.as_bytes())?;
    
    Ok(())
}

pub fn get_text_from_file<P: AsRef<Path>>(file_path: P) -> Result<String, io::Error> {
    // Open the file in read-only mode (ignoring errors for simplicity)
    let mut file = File::open(file_path)?;
    
    // Create a string to store the contents
    let mut contents = String::new();
    
    // Read the file contents into the string
    file.read_to_string(&mut contents)?;
    
    // Return the contents of the file
    Ok(contents)
}