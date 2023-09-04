use clap::Parser;
use libs::Cli;
use libs::Commands;

use commands::qkdir::qkdir;

mod commands;
mod utils;
mod libs;



fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Qkdir(args) => qkdir::execute(args),
    }
    
}
