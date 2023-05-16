mod todo_db;

use clap::{command, Parser, Subcommand};
use crate::todo_db::{Task, TaskError, TodoDb};

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

fn add_task(db: &mut TodoDb, name: String) {
    db.add_task(Task {
        name,
        complete: false,
    })
}

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

fn mark_as_complete(db: &mut TodoDb, task_number: u32) {
    let index = (task_number as usize)
        .checked_sub(1)
        .ok_or(TaskError::NoSuchTask);

    if let Err(e) = index.map(|index| db.mark_as_complete(index)) {
        println!("Cannot mark task as complete: {}", e);
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut db = TodoDb::load(DATA_PATH)?;
    match args.command {
        Command::Add { name } => add_task(&mut db, name),
        Command::List => list_tasks(&db),
        Command::Complete { task_number } => mark_as_complete(&mut db, task_number),
    }

    db.save(DATA_PATH)?;

    Ok(())
}
