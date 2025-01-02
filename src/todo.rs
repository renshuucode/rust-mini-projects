use std::collections::HashMap;
use std::io::Write;

pub struct Todo {
    pub tasks: HashMap<usize, String>,
    pub next_id: usize,
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

    pub fn list_to_buffer(&self, buffer: &mut Vec<u8>) {
        for (id, task) in &self.tasks {
            writeln!(buffer, "{}: {}", id, task).unwrap();
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.tasks.remove(&id);
    }
}