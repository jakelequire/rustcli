use clap::Parser;
use libs::Cli;
use libs::Commands;

use commands::qkdir::qkdir;
use commands::tree::tree;

mod commands;
mod utils;
mod libs;
mod macros;


fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Qkdir(args) => qkdir::execute(args).unwrap(),
        Commands::Tree(args) => tree::execute(args).unwrap(),
    }
}
