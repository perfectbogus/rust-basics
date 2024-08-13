type FnPtr = fn() -> String;

struct Command {
    execute: FnPtr,
    rollback: FnPtr
}


struct Schema {
    commands: Vec<Command>
}

impl Schema {
    fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    fn add_command(&mut self, command: Command) {
        self.commands.push(command)
    }

    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands.iter().rev().map(|cmd| (cmd.execute)()).collect()
    }

}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

fn create_table() -> String {
    "create table".to_string()
}

fn drop_table() -> String {
    "drop table".to_string()
}
fn main() {

    let table_cmd = Command {
        execute: create_table,
        rollback: drop_table
    };

    let field_cmd = Command {
        execute: add_field,
        rollback: remove_field
    };

    let mut schema = Schema::new();
    schema.add_command(table_cmd);
    schema.add_command(field_cmd);

    println!("Executions: {:?}", schema.execute());
    println!("Rollbacks: {:?}", schema.rollback());

}