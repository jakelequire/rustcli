use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct TreeArgs {
    #[command(subcommand)]
    pub command: Option<TreeSubCommands>,
    pub name: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum TreeSubCommands {
    #[command(name = "-bl")]
    #[command(about = "Blacklists a directory from the tree \n Usage: tree -bl <name> \n")]
    Blacklist(BlacklistArgs),

    
    // ... other subcommands
}

#[derive(Args, Debug)]
pub struct BlacklistArgs {
    #[arg(short, long)]
    #[arg(required = false)]
    pub remove: Option<String>,
    #[arg(short, long)]
    #[arg(required = false)]
    pub list: bool,
}
