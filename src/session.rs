use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Session {
    pub username: Option<String>,
    pub current_dir: PathBuf,
    pub history: Vec<String>,
}
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, input: String) -> String;
}


impl Session {
    pub fn load() -> Self {
        fs::read_to_string("session.json")
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("session.json", data).unwrap();
    }
}
