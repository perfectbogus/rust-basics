use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
    assigned_to: Option<String>,
    dependencies: Vec<u32>, // Task IDs this task depends on
}

#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done
}

#[derive(Debug)]
struct Project {
    tasks: HashMap<u32, Task>,
    assignments: HashMap<String, Vec<u32>>,
    next_id: u32,
}

impl Project {
    fn new() -> Project {
        unimplemented!()
    }

    fn add_task(&mut self, title: String, assigned_to: Option<String>) {
        unimplemented!()
    }

    fn add_dependency(&mut self, task_id: u32, depends_on: u32) -> Result<(), String> {
        // Add dependency between tasks
        // Validate both tasks exist
        // Check for circular dependencies
        unimplemented!()
    }

    fn assign_task(&mut self, task_id: u32, user: String) -> Result<(), String> {
        // Assign task to user
        // Update assignments map
        unimplemented!()
    }

    fn update_status(&mut self, task_id: u32, status: TaskStatus) {
        // update task status
        // validate dependencies are done if making as Done
        unimplemented!()
    }

    fn get_blocked_tasks(&self) -> Vec<&Task> {
        // Return task blocked by incomplete dependencies
        unimplemented!()
    }

    fn get_user_tasks(&self, user: &str) -> Vec<&Task> {
        // get all tasks assigned to user
        unimplemented!()
    }

    fn remove_task(&mut self, task_id: u32) -> Result<Task, String> {
        // remove task and clean up dependencies/assignments
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {

    }

    #[test]
    fn test_add_dependency() {}

    #[test]
    fn test_update_status() {}

    #[test]
    fn test_remove_task() {

    }
}






























fn main() {
    println!("Hello, world!");
}
