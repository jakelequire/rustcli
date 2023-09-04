use std::fs::OpenOptions;
use std::io::{Read, Result};
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

pub fn execute() -> Result<()> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("data");
    file_path.push("commands");
    file_path.push("qkdir.json");

    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;


    let data: Directories = serde_json::from_str(&contents)?;

    for dir in &data.directories {
        println!("{}", format!("\n {} \n {}", dir.name, dir.path).green());
    }

    Ok(())
}