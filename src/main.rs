mod todo_db;

use clap::{command, Parser, Subcommand};
use crate::todo_db::{Task, TodoDb};

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

fn list_tasks(db: &TodoDb) {
    let task_list = db.task_list();
    if task_list.is_empty() {
        println!("There are no tasks!");
        return;
    }

    let width = ((task_list.len()).ilog10() + 1) as usize;
    for (i, task) in task_list.iter().enumerate() {
        println!(
            "{number:>width$}. [{complete_marker}] {task}",
            number = i + 1,
            width = width,
            complete_marker = if task.complete { 'x' } else { ' ' },
            task = task.name
        );
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut db = TodoDb::load(DATA_PATH)?;
    match args.command {
        Command::Add { name } => {
            db.add_task(Task {
                name,
                complete: false,
            })
        },
        Command::List => list_tasks(&db),
        _ => todo!()
    }

    db.save(DATA_PATH)?;

    Ok(())
}
