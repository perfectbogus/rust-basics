// Challenge: Shared Todo List
// Your job: Write BOTH the implementation AND the test logic!
use std::rc::Rc;
use std::cell::RefCell;

// TODO: Define your SharedTodoList struct here

// TODO: Define a Task struct (should have text and completed status)
struct Task {
    text: String,
    completed: bool,
}

#[derive(Clone)]
struct SharedTodoList {
    list: Rc<RefCell<Vec<Task>>>
}

impl SharedTodoList {
    fn new() -> Self {
        SharedTodoList {
            list: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_task(&self, text: String) {
        self.list.borrow_mut().push(Task { text, completed: false });
    }

    fn completed_tasks(&self) -> usize {
        self.list.borrow().iter().filter(|task| task.completed).count()
    }

    fn uncompleted_tasks(&self) -> usize {
        self.list.borrow().iter().filter(|task| !task.completed).count()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_todo_list() {
        // TODO: Create a new todo list
        let shared_todo = SharedTodoList::new();

        // These assertions must pass:
        assert_eq!( shared_todo.list.borrow().len() , 0);
        assert_eq!( shared_todo.completed_tasks(), 0);
    }

    #[test]
    fn test_add_tasks() {
        // TODO: Create todo list and add some tasks
        let shared_todo = SharedTodoList::new();

        shared_todo.add_task(String::from("test"));
        shared_todo.add_task(String::from("test2"));


        // These assertions must pass:
        assert_eq!(shared_todo.list.borrow().len(), 2);
        assert_eq!(shared_todo.completed_tasks(), 0);
        assert_eq!(shared_todo.uncompleted_tasks(), 2);
    }

    #[test]
    fn test_complete_tasks() {
        // TODO: Create todo list, add tasks, complete some
        let shared_todo = SharedTodoList::new();

        shared_todo.add_task(String::from("test"));
        shared_todo.add_task(String::from("test2"));
        shared_todo.add_task(String::from("test3"));



        // These assertions must pass:
        // assert_eq!(/* should have 3 total tasks */, 3);
        // assert_eq!(/* should have 1 completed task */, 1);
        // assert_eq!(/* should have 2 pending tasks */, 2);
    }
    //
    // #[test]
    // fn test_shared_todo_list() {
    //     // TODO: Create one todo list, clone it to create shared access
    //
    //     // These assertions must pass:
    //     assert_eq!(/* list1 should show 1 task */, 1);
    //     assert_eq!(/* list2 should show 1 task (same data!) */, 1);
    //     assert_eq!(/* list1 should show 2 tasks after list2 adds one */, 2);
    //     assert_eq!(/* original list should show 2 tasks */, 2);
    // }
    //
    // #[test]
    // fn test_find_task_by_text() {
    //     // TODO: Create todo list with several tasks
    //
    //     // These assertions must pass:
    //     assert!(/* should find task with text "Buy milk" */.is_some());
    //     assert!(/* should NOT find task with text "Nonexistent" */.is_none());
    // }
    //
    // #[test]
    // fn test_remove_completed_tasks() {
    //     // TODO: Create list, add tasks, complete some, then remove completed ones
    //
    //     // These assertions must pass:
    //     assert_eq!(/* should have 2 tasks before cleanup */, 2);
    //     assert_eq!(/* should have 1 task after removing completed */, 1);
    //     assert_eq!(/* should have 0 completed tasks after cleanup */, 0);
    // }
    //
    // #[test]
    // fn test_get_all_task_texts() {
    //     // TODO: Create list with 3 tasks with specific texts
    //
    //     // These assertions must pass:
    //     assert_eq!(/* should return vec with 3 task texts */.len(), 3);
    //     assert!(/* should contain "Task 1" */.contains(&"Task 1".to_string()));
    //     assert!(/* should contain "Task 2" */.contains(&"Task 2".to_string()));
    //     assert!(/* should contain "Task 3" */.contains(&"Task 3".to_string()));
    // }
}

fn main() {
    println!("Hello, world!");
}
