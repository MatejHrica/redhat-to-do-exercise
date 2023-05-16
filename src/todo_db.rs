use std::fs::File;
use std::io;
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
}