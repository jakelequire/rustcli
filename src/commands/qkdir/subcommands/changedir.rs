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

pub fn execute (name: &str) -> Result<()> {
    let mut script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    script_path.push("sh");
    script_path.push("qkdir.bat");

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

    let mut index: Option<usize> = None;

    for (i, dir) in data.directories.iter().enumerate() {
        if dir.name == name {
            index = Some(i);
        }
    }

    match index {
        Some(i) => {
            let nav_path = Some(data.directories[i].path.clone());
            let path: PathBuf = PathBuf::from(nav_path.unwrap());
            println!("{}", path.display());

            println!("DEBUG SCRIPT {}", path.display());
            // Run the script
            Command::new(script_path)
                .spawn()
                .expect("Failed to execute process");

            // Set the current directory for the process
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