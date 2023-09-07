use std::io::Result;
use colored::Colorize;

use crate::commands::tree::args::TreeSubCommands;
use crate::commands::tree::subcommands::blacklist::execute as blacklist_execute;
use crate::commands::tree::subcommands::remove::execute as remove_execute;
use crate::commands::tree::subcommands::list::execute as list_execute;

use crate::commands::tree::subcommands::tree::execute as tree_execute;

use super::args::TreeArgs;

pub fn execute(args: &TreeArgs) -> Result<()> {
    if let Some(command) = &args.command {
        match command {
            // Handle the blacklist subcommand
            TreeSubCommands::Blacklist(blacklist_args) => {
                // If the remove flag is set, remove the directory from the blacklist
                // `tree blacklist --remove <directory_name>`
                if let Some(directory_name) = &blacklist_args.remove {
                    let _ = remove_execute(directory_name).unwrap_or_else(|err: std::io::Error| {
                        println!("{}", format!("Error: {}", err).red());
                    });
                }
                // If the list flag is set, list the blacklisted directories
                // `tree blacklist --list`
                else if blacklist_args.list {
                    println!("{}", "Listing blacklisted directories...".green());
                    let _ = list_execute().unwrap_or_else(|err: std::io::Error| {
                        println!("{}", format!("Error: {}", err).red());
                    });
                }
                // If no flags are set, simply add the directory to the blacklist
                // `tree blacklist <directory_name>`
                else if let Some(name) = &args.name {
                    let _ = blacklist_execute(name).unwrap_or_else(|err: std::io::Error| {
                        println!("{}", format!("Error: {}", err).red());
                    });
                }
            }
        }
    }
    // If no subcommand is provided and the name field is present, 
    else if args.name.is_some() {
        let _ = tree_execute().unwrap_or_else(|err: std::io::Error| {
            println!("{}", format!("Error: {}", err).red());
        });
    } 
    // If neither a subcommand nor a directory name is provided, 
    // execute the default tree command
    else {
        let _ = tree_execute().unwrap_or_else(|err: std::io::Error| {
            println!("{}", format!("Error: {}", err).red());
        });
    }

    Ok(())
}
