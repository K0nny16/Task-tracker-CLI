use clap::Parser;

use std::io::Result;
use note::args::{Args, Command};
use note::task_handler::{complete_task, delete_task, handle_add_task, show_task_list};

fn main() {
    let args = Args::parse();
    if let Err(error) = match_args(args) {
        println!("Something went wrong! \n {}", error);
    }
}

fn match_args(args: Args) -> Result<()> {
    match args.command {
        Command::List => show_task_list(),
        Command::Add {
            description,
            person,
        } => handle_add_task(description, person),
        Command::Done { id } => complete_task(id),
        Command::Delete { id } => delete_task(id),
    }
}
