use std::io::Result;
use std::env;




pub fn execute() -> Result<()> {
    let current_dir = env::current_dir()?;

    println!("{}", format!("{}", current_dir.display()));
    Ok(())
}