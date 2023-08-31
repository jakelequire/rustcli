use clap::Parser;
use types::Cli;
use types::Commands;

mod commands;
mod utils;
mod types;



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Qkdir(args) => commands::qkdir::execute(args),
        Commands::Build(args) => commands::build::execute(args)
    }

}
