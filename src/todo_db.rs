use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::{fmt, io};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use anyhow::Context;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub complete: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
struct Data {
    tasks: Vec<Task>,
}

#[derive(Default)]
pub struct TodoDb {
    data: Data,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TaskError {
    NoSuchTask,
}

impl Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSuchTask => write!(f, "No such task"),
        }
    }
}

impl Error for TaskError {}

impl TodoDb {
    pub fn new() -> Self {
        Default::default()
    }

    /// Load from file.
    /// If the file does not exist, returns an empty TodoDb.
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let data: Data = serde_json::from_reader(reader)
                    .context("Failed to read input file")?;
                Ok(Self { data })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Ok(Self::new())
            }
            Err(e) => return Err(e.into()),
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let file = File::create(path)
            .context("Failed to create output file")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.data)
            .context("Failed to save to output file")?;
        Ok(())
    }

    pub fn task_list(&self) -> &[Task] {
        &self.data.tasks
    }

    pub fn mark_as_complete(&mut self, task_index: usize) -> Result<(), TaskError> {
        if let Some(task) = self.data.tasks.get_mut(task_index) {
            task.complete = true;
            Ok(())
        } else {
            Err(TaskError::NoSuchTask)
        }
    }

    pub fn delete_all_completed_tasks(&mut self) {
        self.data.tasks.retain_mut(|task| { !task.complete });
    }

    pub fn add_task(&mut self, task: Task) {
        self.data.tasks.push(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_empty() {
        let db = TodoDb::new();
        assert!(db.task_list().is_empty());
    }

    #[test]
    fn test_add_one_task() {
        let mut db = TodoDb::new();
        let task = Task {
            name: String::from("Do a thing"),
            complete: false,
        };
        db.add_task(task.clone());

        assert_eq!(db.task_list(), &[task]);
    }

    #[test]
    fn test_mark_task_complete_no_such_task() {
        let mut db = TodoDb::new();
        let result = db.mark_as_complete(1);

        assert_eq!(result, Err(TaskError::NoSuchTask));
    }

    #[test]
    fn test_mark_task_complete_task_exists() {
        let mut db = TodoDb::new();
        let task = Task {
            name: String::from("Do a thing"),
            complete: false,
        };
        let result_task = Task { complete: true, ..task.clone() };

        db.add_task(task);

        let result = db.mark_as_complete(0);
        assert!(result.is_ok());

        assert_eq!(db.task_list(), &[result_task]);
    }

    #[test]
    fn test_delete_all_completed() {
        let mut db = TodoDb::new();
        let t1 = Task { name: String::from("t1"),complete: true};
        let t2 = Task { name: String::from("t2"),complete: false};
        let t3 = Task { name: String::from("t3"),complete: false};
        let t4 = Task { name: String::from("t4"),complete: true};
        db.add_task(t1);
        db.add_task(t2.clone());
        db.add_task(t3.clone());
        db.add_task(t4);

        db.delete_all_completed_tasks();
        assert_eq!(db.task_list(), &[t2, t3]);
    }
}