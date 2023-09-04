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

pub fn execute(name: &str, path: &str) -> Result<()> {
    // C:/Programming/rustcli/data/commands/qkdir.json
    // 0. Get path to JSON file
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("data");
    file_path.push("commands");
    file_path.push("qkdir.json");


    // 1. Read existing JSON file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    

    // 2. Parse JSON into a Directories struct
    let mut data: Directories = serde_json::from_str(&contents)?;

    // Safety checks for duplicates
    for dir in &data.directories {
        if dir.name == name {
            return Err(Error::new(ErrorKind::InvalidInput, "Name already exists"));
        }
        if dir.path == path {
            return Err(Error::new(ErrorKind::InvalidInput, "Path already exists"));
        }
    }

    // 3. Add new directory to Directories struct
    data.directories.push(Directory {
        name: name.to_string(),
        path: path.to_string(),
    });

    // 4. Serialize Directories struct back into JSON
    let updated_contents: String = serde_json::to_string_pretty(&data)?;

    // 5. Truncate and Write updated JSON to file
    file.set_len(0)?;  // Truncate the file to zero length
    file.seek(std::io::SeekFrom::Start(0))?;  // Reset the file pointer to the start
    file.write_all(updated_contents.as_bytes())?;

    println!("{}", format!("Added: \n {} \n {}", name.bold().underline(), path).green());

    Ok(())
}

