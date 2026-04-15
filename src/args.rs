use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(short_flag = 'a', long_flag = "add")]
    Add {
        description: String,

        #[arg(short = 'p', long = "person")]
        person: Option<String>,
    },

    #[command(short_flag = 'd', long_flag = "done")]
    Done { id: usize },

    #[command(long_flag = "delete")]
    Delete { id: usize },

    #[command(short_flag = 'l', long_flag = "list")]
    List,
}
