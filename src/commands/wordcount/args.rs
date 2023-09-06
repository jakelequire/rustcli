use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct Count {
    #[command(subcommand)]
    pub command: Option<QkdirSubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum QkdirSubCommands {
    #[command(name = "-tsx")]
    #[command(about = "")]
    Tsx,

    // ... other subcommands
}

#[derive(Args, Debug)]
pub struct AddArgs {

}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub name: String,
}