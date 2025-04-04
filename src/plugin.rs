use std::collections::HashMap;

pub type CommandFn = fn(Vec<String>);

pub struct CommandRegistry {
    pub commands: HashMap<String, CommandFn>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, handler: CommandFn) {
        self.commands.insert(name.to_string(), handler);
    }

    pub fn execute(&self, name: &str, args: Vec<String>) {
        if let Some(handler) = self.commands.get(name) {
            handler(args);
        } else {
            println!("Command '{}' not found", name);
        }
    }
}
