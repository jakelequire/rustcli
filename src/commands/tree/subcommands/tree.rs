use std::io::Result;
use std::env;
use std::fs;



pub fn execute() -> Result<()> {
    let current_dir = env::current_dir()?;

    // Identify all files and folders in the current directory
    let mut files: Vec<String> = Vec::new();
    let mut folders: Vec<String> = Vec::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let path = path.to_str().unwrap();

        if fs::metadata(path)?.is_dir() {
            folders.push(path.to_string());
        } else {
            files.push(path.to_string());
        }
    }

    // Print the files and folders in the current directory
    for file in files {
        println!("├── {}", file);
    }

    for folder in folders {
        println!("└── {}", folder);
    }

    Ok(())
}
