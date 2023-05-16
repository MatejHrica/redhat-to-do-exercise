mod todo_db;

use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[clap(subcommand)]
    /// The subcommand to run
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {

    /// Add a new task
    Add {
        /// The name of the task
        name: String,
    },

    /// List all tasks
    List,

    /// Mark the task as completed
    Complete {
        /// The number of the task (can be obtained from list subcommand)
        task_number: u32,
    },
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
