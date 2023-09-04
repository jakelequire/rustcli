use clap::{Parser, Subcommand};

use crate::commands::qkdir::args::QkdirArgs;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(name = "qkdir")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "qkdir")]
    Qkdir(QkdirArgs),
}

