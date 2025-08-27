use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindConfiguration {
    modifier_keys: Vec<String>,
    key: Option<String>,
}

impl KeyBindConfiguration {
    pub fn new() -> Self {
        Self {
            modifier_keys: Vec::new(),
            key: None,
        }
    }
    
    pub fn get_key_names(&self) -> Vec<String> {
        let mut keys = self.modifier_keys.clone();
        if let Some(key) = &self.key {
            keys.push(key.clone());
        }
        keys
    }

    pub fn has_key(&self, key_name: String) -> bool {
        if let Some(key) = &self.key {
            if *key == key_name {
                return true;
            }
        }

        if self.modifier_keys.contains(&key_name) {
            return true;
        }

        false
    }

    pub fn append_key(&mut self, key_name: String) {
        if key_name == "CTRL" || key_name == "SHIFT" || key_name == "ALT" {
            self.modifier_keys.push(key_name);
        } else {
            self.key = Some(key_name);
        }
    }
}