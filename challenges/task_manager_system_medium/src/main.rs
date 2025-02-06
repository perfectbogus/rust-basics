use std::collections::{HashMap, HashSet};
use std::vec;

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
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            assignments: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, assigned_to: Option<String>) -> u32 {
        let id = self.next_id;

        if let Some(user) = &assigned_to {
            self.assignments.entry(user.to_string())
                .or_insert_with(Vec::new)
                .push(id);
        }

        self.tasks.insert(id, Task {
            id,
            title,
            status: TaskStatus::Todo,
            assigned_to,
            dependencies: Vec::new()
        });
        self.next_id += 1;
        id
    }

    fn add_dependency(&mut self, task_id: u32, depends_on: u32) -> Result<(), String> {
        // Add dependency between tasks
        // Validate both tasks exist
        // Check for circular dependencies
        if !self.tasks.contains_key(&task_id) {
            return Err(format!("Task with id {} not found", task_id));
        }

        if !self.tasks.contains_key(&depends_on) {
            return Err(format!("Task depends on with id {} not found", depends_on));
        }

        // Check for circular dependencies
        let mut visited = HashSet::new();
        let mut stack = vec![depends_on];

        while let Some(current) = stack.pop() {
            if current == task_id {
                return Err("Circular dependency detected".to_string());
            }
            if visited.insert(current) {
                if let Some(task) = self.tasks.get(&current) {
                    stack.extend(&task.dependencies);
                }
            }
        }

        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.dependencies.push(depends_on);
            Ok(())
        } else {
            Err("Failed to update task".to_string())
        }

    }

    fn assign_task(&mut self, task_id: u32, user: String) -> Result<(), String> {
        // Assign task to user
        // Update assignments map
        let mut task = self.tasks.get_mut(&task_id).unwrap();

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

        assert!(project.add_dependency(task2, task1).is_ok());
        assert!(project.tasks.get(&task2).unwrap().dependencies.contains(&task1));
    }

    #[test]
    fn test_circular_dependencies() {
        let mut project = Project::new();
        let task1 = project.add_task("Task 1".to_string(), None);
        let task2 = project.add_task("Task 2".to_string(), None);
        let task3 = project.add_task("Task 3".to_string(), None);

        //Create dependency chain: task1 -> task2 -> task3
        assert!(project.add_dependency(task2, task1).is_ok());
        assert!(project.add_dependency(task3, task2).is_ok());

        // Try to create circular dependency: task3 -> task1
        assert!(project.add_dependency(task1, task3).is_err());

        // Verify original dependencies still intact
        assert!(project.tasks.get(&task2).unwrap().dependencies.contains(&task1));
        assert!(project.tasks.get(&task3).unwrap().dependencies.contains(&task2));
    }

    #[test]
    fn test_assign_task() {
        let mut project = Project::new();
        let task_id = project.add_task("Task 1".to_string(), None);

        // Test successful assignment
        assert!(project.assign_task(task_id, "user1".to_string()).is_ok());
        assert_eq!(project.tasks.get(&task_id).unwrap().assigned_to.as_ref().unwrap(), "user1");
        assert!(project.assignments.get("user1").unwrap().contains(&task_id));

        // Test assigning non-existent task
        assert!(project.assign_task(999, "user1".to_string()).is_err());

        // Test reassigning task
        assert!(project.assign_task(task_id, "user2".to_string()).is_ok());
        assert_eq!(project.tasks.get(&task_id).unwrap().assigned_to.as_ref().unwrap(), "user2");
        assert!(!project.assignments.get("user1").unwrap().contains(&task_id));
        assert!(project.assignments.get("user2").unwrap().contains(&task_id));
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
