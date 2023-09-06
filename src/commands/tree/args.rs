use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct TreeArgs {
    #[command(subcommand)]
    pub command: Option<TreeSubCommands>,
    pub tree: Option<bool>,
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
    pub name: String,
}
