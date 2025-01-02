use std::collections::HashMap;

pub struct Todo {
    tasks: HashMap<usize, String>,
    next_id: usize,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, task: String) {
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    pub fn list(&self) {
        for (id, task) in &self.tasks {
            println!("{}: {}", id, task);
        }
    }
}