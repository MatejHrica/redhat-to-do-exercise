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