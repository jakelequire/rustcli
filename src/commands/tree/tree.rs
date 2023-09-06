

use crate::commands::tree::args::TreeSubCommands;
use crate::commands::tree::subcommands::blacklist::execute as blacklist_execute;


pub fn execute() {
    if let Some(command) = &args.command {
        match command {
            // Add a new directory to the list
            // `qkdir -a <name> <path>`
            TreeSubCommands::Blacklist(_) => {
                let _ = blacklist_execute().unwrap_or_else(|err: std::io::Error| {
                    println!("{}", format!("Error: {}", err).red());
                });
            }
        }
    }


}