use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    let mut todo_list = TodoList::new();
    let file_path = Path::new("todo.txt");

    if file_path.exists() {
        if let Err(e) = todo_list.load_from_file(file_path) {
            println!("Error loading tasks: {}", e)
        }
    }

    loop {
        println!("\n--- Todo List ---\n");
        println!("1. Add Task");
        println!("2. Complete Task");
        println!("3. Remove Task");
        println!("4. List Tasks");
        println!("5. Save and exit");

        let mut choice = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut description = String::new();
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();
                let id = todo_list.add_task(description.trim().to_string());
                println!("Task added with ID: {}", id);
            },
            "2" => {
                let mut id_str = String::new();
                print!("Enter task ID to complete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse::<usize>() {
                    if todo_list.complete_task(id) {
                        println!("Task completed");
                    } else {
                        println!("Task not found");
                    }
                } else {
                    println!("Invalid ID!");
                }
            },
            "3" => {
                let mut id_str = String::new();
                print!("Enter task ID to remove:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).unwrap();

                if let Ok(id) = id_str.trim().parse::<usize>() {
                    if todo_list.remove_task(id) {
                        println!("Task removed");
                    } else {
                        println!("Task not found");
                    }
                } else {
                    println!("Invalid ID!");
                }
            },
            "4" => {
                println!("\nTasks: ");
                for task in todo_list.list_tasks() {
                    println!(
                        "ID: {}, Status: {}, Description: {}",
                        task.id,
                        if task.completed { "Completed" } else { "Pending" },
                        task.description
                    )
                }
            },
            "5" => {
                if let Err(e) = todo_list.save_to_file(file_path) {
                    println!("Error saving tasks: {}", e);
                } else {
                    println!("Task saved successfully!");
                }
                break;
            },
            _ => println!("Invalid choice!")
        }

    }
}


#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TodoList {

    fn load_from_file(&mut self, path: &Path) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.tasks.clear();
        self.next_id = 1;

        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(3, ',').collect();
            if parts.len() == 3 {
                if let Ok(id) = parts[0].parse::<usize>() {
                    let completed = parts[1] == "true";
                    let description = parts[2].to_string();

                    self.tasks.insert(
                        id,
                        Task {
                            id,
                            description,
                            completed,
                        },
                    );
                    if id >= self.next_id {
                        self.next_id += 1;
                    }
                }
            }
        }
        Ok(())
    }

    fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        for task in self.tasks.values() {
            writeln!(
                file,
                "{},{},{}",
                task.id, task.description, task.completed
            )?;
        }
        Ok(())
    }

    fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn remove_task(&mut self, id: usize) -> bool {
        self.tasks.remove(&id).is_some()
    }

    fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = true;
            true
        } else {
            false
        }
    }

    fn new() -> Self {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) -> usize {
        let id = self.next_id;
        self.tasks.insert(
            id,
            Task {
                id,
                description,
                completed: false,
            }
        );
        self.next_id += 1;
        id
    }
}


