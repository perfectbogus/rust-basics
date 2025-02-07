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
        let task = self.tasks.get_mut(&task_id)
            .ok_or(format!("Task {} not found", task_id))?;

        if let Some(prev_user) = &task.assigned_to {
            if let Some(tasks) = self.assignments.get_mut(prev_user) {
                tasks.retain(|&id| id != task_id);
            }
        }

        task.assigned_to = Some(user.clone());
        self.assignments.entry(user)
            .or_insert_with(Vec::new)
            .push(task_id);

        Ok(())
    }

    fn update_status(&mut self, task_id: u32, status: TaskStatus) -> Result<(), String>{
        // update task status
        // validate dependencies are done if making as Done
        let dependencies = if let Some(task) = self.tasks.get(&task_id) {
            task.dependencies.clone()
        } else {
            return Err(format!("Task {} not found", task_id));
        };

        // Check dependencies if making as Done
        if status == TaskStatus::Done {
            for dep_id in &dependencies {
                let dep_task = self.tasks.get(dep_id)
                    .ok_or(format!("Task {} not found", dep_id))?;

                if dep_task.status != TaskStatus::Done {
                    return Err(format!("Task {} depends on with id {} but not done", dep_id, dep_task.id));
                }
            }
        }

        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = status;
            Ok(())
        } else {
            Err(format!("Task {} not found", task_id))
        }
    }

    fn get_blocked_tasks(&self) -> Vec<&Task> {
        // Return task blocked by incomplete dependencies
        let mut blocked_tasks = Vec::new();
        for task in self.tasks.values() {
            for dep_in in &task.dependencies {
                if let Some(dep_task) = self.tasks.get(&dep_in) {
                    if dep_task.status != TaskStatus::Done {
                        blocked_tasks.push(task);
                        break;
                    }
                }
            }
        }
        blocked_tasks
    }

    fn get_user_tasks(&self, user: &str) -> Vec<&Task> {
        // get all tasks assigned to user
        self.assignments.get(user)
            .map_or(Vec::new(), |task_ids|
                task_ids.iter()
                    .filter_map(|id| self.tasks.get(id))
                    .collect()
            )
        // self.tasks.iter()
        //     .filter(|&(_, task)| {
        //         task.assigned_to == Some(user.to_string())
        //     })
        //     .map(|(_, task)| task)
        //     .collect()

        // self.tasks.iter()
        //     .filter_map(|(id, task)| {
        //         if task.assigned_to == Some(user.to_string()) {
        //             Some(task)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect::<Vec<&Task>>()


        // self.tasks.iter()
        //     .filter_map(|(task_id, task)| {
        //         match &task.assigned_to {
        //             None => { None }
        //             Some(assigned) => {
        //                 if assigned == user {
        //                     return Some(task)
        //                 }
        //                 None
        //             }
        //         }
        //     })
        //     .collect::<Vec<&Task>>()

        // self.tasks.iter()
        //     .filter_map(|(_, task)| {
        //         if let Some(assigned_to) = &task.assigned_to {
        //             if user == assigned_to {
        //                 return Some(task)
        //             }
        //         }
        //         None
        //     })
        //     .collect()



        // let mut user_tasks = Vec::new();
        // for task in self.tasks.values() {
        //     if task.assigned_to == Some(user.to_string()) {
        //         user_tasks.push(task);
        //     }
        // }
        // user_tasks
    }

    fn remove_task(&mut self, task_id: u32) -> Result<Task, String> {
        // remove task and clean up dependencies/assignments
        if self.tasks.values().any(|t| t.dependencies.contains(&task_id)) {
            return Err(format!("Cannot remove task {}: other tasks depend on it", task_id));
        }

        let task = self.tasks.remove(&task_id)
            .ok_or(format!("Task {} not found", task_id))?;

        if let Some(user) = &task.assigned_to {
            if let Some(tasks) = self.assignments.get_mut(user) {
                tasks.retain(|&id| id != task_id);
            }
        }

        Ok(task)
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
    fn test_get_blocked_tasks() {
        let mut project = Project::new();

        // Create task with dependencies
        let task1 = project.add_task("Task 1".to_string(), None);
        let task2 = project.add_task("Task 2".to_string(), None);
        let task3 = project.add_task("Task 3".to_string(), None);

        project.add_dependency(task2, task1).unwrap();
        project.add_dependency(task3, task2).unwrap();

        // Task 2 and 3 should be blocked initially
        let blocked = project.get_blocked_tasks();
        assert_eq!(blocked.len(), 2);
        assert!(blocked.iter().any(|t| t.id == task2));
        assert!(blocked.iter().any(|t| t.id == task3));

        // Complete task1, task2 should be unblocked but task3 still blocked
        project.update_status(task1, TaskStatus::Done).unwrap();
        let blocked = project.get_blocked_tasks();
        assert_eq!(blocked.len(), 1);
        assert!(blocked.iter().any(|t| t.id == task3));

        // Complete task2, nothing should be blocked
        project.update_status(task2, TaskStatus::Done).unwrap();
        assert!(project.get_blocked_tasks().is_empty());
    }

    #[test]
    fn test_remove_task() {
        let mut project = Project::new();

        // Add tasks with dependencies and assignments
        let task1 = project.add_task("Task 1".to_string(), Some("user1".to_string()));
        let task2 = project.add_task("Task 2".to_string(), None);
        project.add_dependency(task2, task1).unwrap();

        // Remove task1 - should fail due to dependency
        assert!(project.remove_task(task1).is_err());

        // Remove task2 first
        let removed = project.remove_task(task2).unwrap();
        assert_eq!(removed.id, task2);

        // Now task1 can be removed
        let removed = project.remove_task(task1).unwrap();
        assert_eq!(removed.id, task1);

        // Verify cleanup
        assert!(!project.tasks.contains_key(&task1));
        assert!(project.assignments.get("user1").unwrap().is_empty());
    }

    #[test]
    fn test_user_tasks() {
        let mut project = Project::new();
        let task1 = project.add_task("Task 1".to_string(), Some("user1".to_string()));
        let task2 = project.add_task("Task 2".to_string(), Some("user2".to_string()));
        let task3 = project.add_task("Task 3".to_string(), Some("user3".to_string()));

        assert_eq!(project.get_user_tasks("user1").len(), 1);
        assert_eq!(project.get_user_tasks("user2").len(), 1);
    }

}

fn main() {
    println!("Hello, world!");
}
