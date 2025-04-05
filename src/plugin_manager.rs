use std::collections::HashMap;
use crate::plugin::Plugin;

pub struct PluginManager {
    pub plugins: HashMap<String, Box<dyn Plugin>>, // `plugins` made public
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin); // ✅ convert &str to String
    }

    pub fn execute_command(&self, name: &str, input: &str) -> Option<String> {
        self.plugins.get(name).map(|p| p.execute(input.to_string())) // ✅ convert &str to String
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    pub fn get_registered_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
