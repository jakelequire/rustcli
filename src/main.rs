use clap::Parser;
use types::Cli;
use types::Commands;

mod commands;
mod utils;
mod types;



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(name) => {
            println!("'myapp add' was used, name is: {:?}", name.name)
        }
    }

}
