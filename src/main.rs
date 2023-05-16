mod todo_db;

use clap::{command, Parser, Subcommand};
use crate::todo_db::TodoDb;

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

const DATA_PATH: &str = "todo_data.json";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let db = TodoDb::load(DATA_PATH)?;
    db.save(DATA_PATH)?;

    Ok(())
}
