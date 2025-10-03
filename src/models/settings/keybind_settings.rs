use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct KeyBindSettings {
    pub program_keybinds: HashMap<SystemKeybind, KeyBindConfiguration>,
    pub custom_keybinds: HashMap<String, CustomKeybind>
}