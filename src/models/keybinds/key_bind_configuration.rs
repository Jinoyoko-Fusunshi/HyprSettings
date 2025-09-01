use serde::{Deserialize, Serialize};
use crate::ui::controls::keybinds::{ALT_KEY, CONTROL_KEY, SHIFT_KEY, SUPER_KEY};

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

    pub fn append_key(&mut self, key_name: String) {
        if key_name == CONTROL_KEY || key_name == SHIFT_KEY || key_name == ALT_KEY || key_name == SUPER_KEY {
            self.modifier_keys.push(key_name);
        } else {
            self.key = Some(key_name);
        }
    }

    pub fn get_key_names(&self) -> Vec<String> {
        let mut keys = self.modifier_keys.clone();
        if let Some(key) = &self.key {
            keys.push(key.clone());
        }
        keys
    }

    pub fn get_modifier_keys(&self) -> Vec<String> {
        self.modifier_keys.clone()
    }

    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
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
}