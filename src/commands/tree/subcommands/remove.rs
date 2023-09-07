use std::io::{Result, Read, Write, Seek};
use std::fs::OpenOptions;
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use colored::*;

use crate::data_from_file;

#[derive(Serialize, Deserialize, Debug)]
struct Folder {
    directory: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Blacklist {
    blacklist: Vec<Folder>,
}

/// Removes a saved directory from the blacklist
pub fn execute(name: &String) -> Result<()> {
    // Sets the path to the tree.json file
    let file = data_from_file!("tree.json");
    
    // Opens the file (read/write)
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file)?;

    // Reads the contents of the file and stores it in a string
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializes the string into a Blacklist struct
    let mut data: Blacklist = serde_json::from_str(&contents)?;

    // If the blacklist is empty, initialize it with a Folder
    if let Some(folder) = data.blacklist.first_mut() {
        // If the directory is in the blacklist, remove it
        if let Some(pos) = folder.directory.iter().position(|x| x == name) {
            folder.directory.remove(pos);
            println!("{}", format!("{} has been removed from the blacklist", name).green().italic());
        } else {
            println!("{} is not in the blacklist", name);
        }
    } else {
        println!("Blacklist is empty");
    }

    // Serializes the data into a string
    let updated_contents: String = serde_json::to_string_pretty(&data)?;

    // Writes the string to the file
    file.set_len(0)?;
    // Sets the cursor to the beginning of the file
    file.seek(std::io::SeekFrom::Start(0))?;
    // Writes the string to the file
    file.write_all(updated_contents.as_bytes())?;

    Ok(())
}
