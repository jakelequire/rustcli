use std::io::Read;
use std::fs::File;
use std::error::Error;

pub fn execute(file: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut word_count = 0;
    let mut line_count = 0;
    let mut char_count = 0;

    for line in contents.lines() {
        line_count += 1;
        char_count += line.len();
        word_count += line.split_whitespace().count();
    }

    println!("{} {} {}", line_count, word_count, char_count);

    Ok(())
}
