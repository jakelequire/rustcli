use std::fs::OpenOptions;
use std::io::{Read, Result};
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

pub fn execute() -> Result<()> {
    // Sets the path to the tree.json file
    let file = data_from_file!("tree.json");

    // Opens the file (readonly)
    let mut file = OpenOptions::new()
        .read(true)
        .open(&file)?;

    // Reads the contents of the file and stores it in a string
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializes the string into a Blacklist struct
    let mut data: Blacklist = serde_json::from_str(&contents)?;

    // Log the list of blacklisted directories
    if let Some(folder) = data.blacklist.first_mut() {
        for dir in &folder.directory {
            println!("{}", format!("\n {}", dir).green());
        }
    } else {
        println!("Blacklist is empty");
    }

    Ok(())
}