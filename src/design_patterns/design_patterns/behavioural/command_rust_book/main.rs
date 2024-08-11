
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;

impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct CreateIndex;

impl Migration for CreateIndex {
    fn execute(&self) -> &str {
        "create index"
    }

    fn rollback(&self) -> &str {
        "drop index"
    }
}
pub struct AddField;

impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>
}

impl Schema {
    fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

fn main() {
    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);
    let cmd = Box::new(CreateIndex);
    schema.add_migration(cmd);

    let result = schema.execute();
    println!("execution: {:?}", result);

    let rollback_result = schema.rollback();
    println!("rollback: {:?}", rollback_result);
}