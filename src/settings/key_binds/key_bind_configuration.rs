use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindConfiguration {
    key_names: Vec<String>
}

impl KeyBindConfiguration {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            key_names: keys
        }
    }
    
    pub fn get_key_names(&self) -> &Vec<String> {
        &self.key_names
    }
}