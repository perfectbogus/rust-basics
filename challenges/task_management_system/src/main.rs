#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    subtasks: Vec<Box<Task>>
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            subtasks: Vec::new()
        }
    }

    fn get_highest_id(&self) -> u32 {
        let mut max_id = self.id;
        for task in &self.subtasks {
            max_id = max_id.max(task.get_highest_id());
        }
        max_id
    }

    fn add_subtask(&mut self, description: String) -> u32 {
        let id = self.get_highest_id() + 1;
        let new_task = Task::new(id, description);
        self.subtasks.push(Box::new(new_task));
        id
    }

    fn complete(&mut self) {
        self.completed = true;
        for task in &mut self.subtasks {
            task.complete();
        }
    }

    fn find_task(&mut self, id: u32) -> Option<&mut Task>{
        if self.id == id {
           return Some(self);
        }
        for task in &mut self.subtasks {
            if let Some(found) = task.find_task(id) {
                return Some(found);
            }
        }
        None
    }

    fn count_incomplete(&self) -> u32 {
        let mut count = 0;
        for task in &self.subtasks {
            count += task.count_incomplete();
        }
        if !self.completed {
            count += 1;
        }
        count
    }

    fn remove_completed(&mut self) {
        for task in &mut self.subtasks {
            task.remove_completed();
        }
        self.subtasks.retain(|task| !task.completed);
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task::new(1, "Main task".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Main task");
        assert!(!task.completed);
        assert!(task.subtasks.is_empty());
    }

    #[test]
    fn test_add_subtasks() {
        let mut task = Task::new(1, "Main task".to_string());
        let subtask_id = task.add_subtask("Subtask 1".to_string());
        assert_eq!(subtask_id, 2);
        assert_eq!(task.subtasks.len(), 1);

        let subtask_id2 = task.add_subtask("Subtask 2".to_string());
        assert_eq!(subtask_id2, 3);
        assert_eq!(task.subtasks.len(), 2);
    }

    #[test]
    fn test_complete_task() {
        let mut task = Task::new(1, "Main task".to_string());
        task.add_subtask("Subtask 1".to_string());
        task.complete();
        assert!(task.completed);
        assert!(task.subtasks[0].completed);
    }

    #[test]
    fn test_find_task() {
        let mut task = Task::new(1, "Main task".to_string());
        let subtask_id = task.add_subtask("Subtask 1".to_string());
        let found_task = task.find_task(subtask_id);
        assert!(found_task.is_some());
        assert_eq!(found_task.unwrap().description, "Subtask 1");
    }

    #[test]
    fn test_count_incomplete() {
        let mut task = Task::new(1, "Main task".to_string());
        task.add_subtask("Subtask 1".to_string());
        task.add_subtask("Subtask 2".to_string());
        assert_eq!(task.count_incomplete(), 3); // Main task + 2 subtasks

        task.find_task(2).unwrap().complete();
        assert_eq!(task.count_incomplete(), 2);
    }

    #[test]
    fn test_remove_completed() {
        let mut task = Task::new(1, "Main task".to_string());
        task.add_subtask("Subtask 1".to_string());
        task.add_subtask("Subtask 2".to_string());

        task.find_task(2).unwrap().complete();
        task.remove_completed();
        assert_eq!(task.subtasks.len(), 1);
        assert_eq!(task.subtasks[0].description, "Subtask 2");
    }
}