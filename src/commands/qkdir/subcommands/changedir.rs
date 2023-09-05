use std::fs::OpenOptions;
use std::io::{Read, Result, Error, ErrorKind};
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::env;

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
/// Changes the current directory to the directory with the given name
/// 
/// name: &str - The name of the directory to change to
/// 
/// Returns a [Result<()>] indicating success or failure
/// 
/// `qkdir <name>`
///
pub fn execute (name: &str) -> Result<()> {
    // Sets the path to the qkdir.json file
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("data");
    file_path.push("commands");
    file_path.push("qkdir.json");

    // Opens the file (read only)
    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)?;

    // Reads the contents of the file and stores it in a string
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    // Deserializes the string into a Directories struct
    let data: Directories = serde_json::from_str(&contents)?;

    // Searches for the index of the directory with the given name
    let mut index: Option<usize> = None;

    // Iterates through the directories and checks if the name matches
    for (i, dir) in data.directories.iter().enumerate() {
        if dir.name == name {
            // If the name matches, set the index to the current index
            index = Some(i);
        }
    }

    // If the index is not None, then the name exists
    match index {
        Some(i) => {
            // Sets the path to the directory
            let nav_path = Some(data.directories[i].path.clone());
            let path: PathBuf = PathBuf::from(nav_path.unwrap());

            // Opens a new cmd window and navigates to the directory
            Command::new("cmd")
                .arg("/c")
                .arg("start")
                .arg("cmd")
                .arg("/k")
                .arg(format!("cd {}", path.display()))
                .spawn()?;
        

            // If an error occurs while changing the current directory, return the error
            if let Err(e) = env::set_current_dir(&path) {
                return Err(Error::new(ErrorKind::Other, format!("Failed to change directory: {}", e)));
            }
        }
        None => {
            return Err(Error::new(ErrorKind::InvalidInput, "Name does not exist"));
        }
    }

    Ok(())
}
