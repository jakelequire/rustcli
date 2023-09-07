use std::io::{Result, Read, Write, Seek};
use std::fs::OpenOptions;
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize, Serialize};
use crate::data_from_file;

#[derive(Serialize, Deserialize, Debug)]
struct Folder {
    directory: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Blacklist {
    blacklist: Vec<Folder>,
}

pub fn execute(name: &String) -> Result<()> {
    let path = data_from_file!("tree.json");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let mut data: Blacklist = serde_json::from_str(&contents)?;

    if data.blacklist.is_empty() {
        // If the blacklist is empty, initialize it with a Folder
        data.blacklist.push(Folder {
            directory: vec![name.clone()],
        });
    } else {
        // If the directory is not already in the blacklist, add it
        if !data.blacklist[0].directory.contains(name) {
            data.blacklist[0].directory.push(name.clone());
        }
    }

    let new_data = serde_json::to_string_pretty(&data)?;

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(new_data.as_bytes())?;

    Ok(())
}
