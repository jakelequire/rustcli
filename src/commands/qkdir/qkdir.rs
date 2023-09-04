use crate::commands::qkdir::args::QkdirArgs;
use crate::commands::qkdir::args::QkdirSubCommands;

use crate::commands::qkdir::subcommands::add::execute as add_execute;
use crate::commands::qkdir::subcommands::list::execute as list_execute;
use crate::commands::qkdir::subcommands::remove::execute as remove_execute;

use colored::*;


pub fn execute(args: &QkdirArgs) {


    match &args.command {
        QkdirSubCommands::Add(add_args) => {
            let name: &String = &add_args.name;
            let path: &String = &add_args.path;

            let _ = add_execute(name, path).unwrap_or_else(|err: std::io::Error| {
                println!("{}", format!("Error: {}", err).red());
            });
        }

        QkdirSubCommands::List => {
            let _ = list_execute().unwrap();
        }

        QkdirSubCommands::Remove(remove_args) => {
            let name: &String = &remove_args.name;
            let _ = remove_execute(name).unwrap_or_else(|err: std::io::Error| {
                println!("{}", format!("Error: {}", err).red());
            });
        }

    }

}
