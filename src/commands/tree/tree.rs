use std::io::Result;
use colored::Colorize;

use crate::commands::tree::args::TreeSubCommands;
use crate::commands::tree::subcommands::blacklist::execute as blacklist_execute;
use crate::commands::tree::subcommands::tree::execute as tree_execute;

use super::args::TreeArgs;

pub fn execute(args: &TreeArgs) -> Result<()>{
    if let Some(command) = &args.command {
        match command {
            // Add a new directory to the list
            // `tree -bl <folder_name>`
            TreeSubCommands::Blacklist(_) => {
                let _ = blacklist_execute().unwrap_or_else(|err: std::io::Error| {
                    println!("{}", format!("Error: {}", err).red());
                });
            }
        }
    } else if let Some(tree) = &args.tree {
        if *tree {
            let _ = tree_execute().unwrap_or_else(|err: std::io::Error| {
                println!("{}", format!("Error: {}", err).red());
            });
        }
    }

    Ok(())
}
