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

pub fn execute(name: &str) -> Result<()> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("data");
    file_path.push("commands");
    file_path.push("qkdir.json");


    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let mut data: Directories = serde_json::from_str(&contents)?;

    let mut index: Option<usize> = None;

    for (i, dir) in data.directories.iter().enumerate() {
        if dir.name == name {
            index = Some(i);
        }
    }

    match index {
        Some(i) => {
            data.directories.remove(i);
        }
        None => {
            return Err(Error::new(ErrorKind::InvalidInput, "Name does not exist"));
        }
    }

    let updated_contents: String = serde_json::to_string_pretty(&data)?;

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(updated_contents.as_bytes())?;

    println!("{}", format!("Removed {}", name).green());

    Ok(())
}