use clap::{Args, Parser, Subcommand};

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

#[derive(Args, Debug)]
pub struct QkdirArgs {
    #[arg(required = false)]
    pub name: String,
    #[arg(required = false)]
    pub path: Option<String>,
}

