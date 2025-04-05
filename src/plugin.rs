use std::collections::HashMap;

#[allow(dead_code)]
pub type CommandFn = fn(Vec<String>);

#[allow(dead_code)]
pub struct CommandRegistry {
    pub commands: HashMap<String, CommandFn>,
}

#[allow(dead_code)]
impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
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

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;


    #[allow(dead_code)]
    fn description(&self) -> &'static str;

    fn execute(&self, input: String) -> String;
}
