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

    fn add_task(&mut self, title: String, assigned_to: Option<String>) -> u32 {
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

    fn update_status(&mut self, task_id: u32, status: TaskStatus) -> Result<(), String>{
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
        let mut project = Project::new();
        let id = project.add_task("Test task".to_string(), Some("user1".to_string()));
        assert!(project.tasks.contains_key(&id));
        assert_eq!(project.assignments.get("user1").unwrap().len(), 1);
    }

    #[test]
    fn test_add_dependency() {
        let mut project = Project::new();
        let task1 = project.add_task("Task 1".to_string(), None);
        let task2 = project.add_task("Task 2".to_string(), None);

        assert!(project.add_dependency(task1, task2).is_ok());
        assert!(project.tasks.get(&task2).unwrap().dependencies.contains(&task1));
    }

    #[test]
    fn test_update_status() {
        let mut project = Project::new();
        let task1 = project.add_task("Task 1".to_string(), None);
        let task2 = project.add_task("Task 2".to_string(), None);
        project.add_dependency(task2, task1).unwrap();

        // Can't complete task2 before task1
        assert!(project.update_status(task2, TaskStatus::Done).is_err());

        project.update_status(task1, TaskStatus::Done).unwrap();
        assert!(project.update_status(task2, TaskStatus::Done).is_ok());
    }

    #[test]
    fn test_remove_task() {
        let mut project = Project::new();
        let task1 = project.add_task("Task 1".to_string(), Some("user1".to_string()));

        let removed = project.remove_task(task1).unwrap();

        assert!(!project.tasks.contains_key(&task1));
        assert!(project.assignments.get("user1").unwrap().is_empty());
    }
}






























fn main() {
    println!("Hello, world!");
}
