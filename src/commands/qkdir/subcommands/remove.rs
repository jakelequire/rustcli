use std::fs::OpenOptions;
use std::io::{Read, Write, Result, Seek, Error, ErrorKind};
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
struct Directory {
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Directories {
    directories: Vec<Directory>,
}


/// 
/// Removes a saved directory from the list
/// 
/// name: &str - The name of the directory to remove
/// 
/// Returns a [Result<()>] indicating success or failure
/// 
/// `qkdir -r <name>`
/// 
pub fn execute(name: &str) -> Result<()> {
    // Sets the path to the qkdir.json file
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("data");
    file_path.push("commands");
    file_path.push("qkdir.json");


    // Opens the file (read/write)
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    // Reads the contents of the file and stores it in a string
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializes the string into a Directories struct
    let mut data: Directories = serde_json::from_str(&contents)?;

    // Searches for the index of the directory with the given name
    let mut index: Option<usize> = None;

    // Iterates through the directories and checks if the name matches
    for (i, dir) in data.directories.iter().enumerate() {
        if dir.name == name {
            index = Some(i);
        }
    }

    // If the index is not None, then the name exists
    match index {
        Some(i) => {
            // Removes the directory from the list
            data.directories.remove(i);
        }
        None => {
            // If the index is None, then the name does not exist
            return Err(Error::new(ErrorKind::InvalidInput, "Name does not exist"));
        }
    }

    // Serializes the Directories struct into a string
    let updated_contents: String = serde_json::to_string_pretty(&data)?;

    // Sets the length of the file to 0
    file.set_len(0)?;
    // Sets the cursor to the start of the file
    file.seek(std::io::SeekFrom::Start(0))?;
    // Writes the updated contents to the file
    file.write_all(updated_contents.as_bytes())?;

    println!("{}", format!("Removed {}", name).green());

    Ok(())
}
