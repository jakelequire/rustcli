use clap::{Args, Subcommand};


#[derive(Args, Debug)]
pub struct QkdirArgs {
    #[command(subcommand)]
    pub command: QkdirSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum QkdirSubCommands {
    #[command(name = "-a")]
    #[command(about = "Add a new directory to the list \n Usage: qkdir -a <name> <path> \n")]
    Add(AddArgs),

    #[command(name = "-l")]
    #[command(about = "List all directories in the list \n Usage: qkdir -l \n")]
    List,

    #[command(name = "-r")]
    #[command(about = "Remove a directory from the list \n Usage: qkdir -r <name> \n")]
    Remove(RemoveArgs),
    
    // ... other subcommands
}

#[derive(Args, Debug)]
pub struct AddArgs {
    pub name: String,
    pub path: String,
}


#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub name: String,
}