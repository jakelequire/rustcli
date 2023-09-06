use std::io::Result;
use crate::commands::qkdir::args::QkdirArgs;
use crate::commands::qkdir::args::QkdirSubCommands;

use crate::commands::qkdir::subcommands::add::execute as add_execute;
use crate::commands::qkdir::subcommands::list::execute as list_execute;
use crate::commands::qkdir::subcommands::remove::execute as remove_execute;
use crate::commands::qkdir::subcommands::changedir::execute as changedir_execute;

use colored::*;
/// 
/// Execute the qkdir command
/// 
/// # Arguments
/// 
/// * `args` - The arguments passed to the qkdir command
/// 
/// # Example
/// 
/// ```
/// use crate::commands::qkdir::qkdir::execute;
/// 
/// let args = QkdirArgs {
///    command: None,
///   name: Some(String::from("test")),
/// };
/// 
/// execute(&args);
/// ```
/// 
pub fn execute(args: &QkdirArgs) -> Result<()> {
    // If the command is not None, then a subcommand was passed
    if let Some(command) = &args.command {
        match command {
            // Add a new directory to the list
            // `qkdir -a <name> <path>`
            QkdirSubCommands::Add(add_args) => {
                let name: &String = &add_args.name;
                let path: &String = &add_args.path;

                let _ = add_execute(name, path).unwrap_or_else(|err: std::io::Error| {
                    println!("{}", format!("Error: {}", err).red());
                });
            }

            // List all directories in the list
            // `qkdir -l`
            QkdirSubCommands::List => {
                let _ = list_execute().unwrap();
            }

            // Remove a directory from the list
            // `qkdir -r <name>`
            QkdirSubCommands::Remove(remove_args) => {
                let name: &String = &remove_args.name;
                let _ = remove_execute(name).unwrap_or_else(|err: std::io::Error| {
                    println!("{}", format!("Error: {}", err).red());
                });
            }
        }
    } else if let Some(name) = &args.name {
        println!("{}", format!("Navigating to {}", name).green());
        // Change the current working directory to the directory with the given saved name
        // `qkdir <name>`
        let _ = changedir_execute(name).unwrap_or_else(|err: std::io::Error| {
            println!("{}", format!("Error: {}", err).red());
        });
    }

    Ok(())
}
