type Migration<'a> = Box<dyn Fn() -> &'a str>;

struct Schema<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>
}

impl<'a> Schema<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![]
        }
    }

    fn add_command<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }

    fn execute_commands(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }

    fn execute_rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn create_table() -> &'static str {
    "create table"
}

fn drop_table() -> &'static str {
    "drop table"
}

fn create_field() -> &'static str {
    "create field"
}

fn drop_field() -> &'static str {
    "drop field"
}

fn main() {

    let mut schema = Schema::new();

    schema.add_command(create_table, drop_table);
    schema.add_command(create_field, drop_field);

    println!("Executions: {:?}", schema.execute_commands());
    println!("Rollbacks: {:?}", schema.execute_rollback());
}