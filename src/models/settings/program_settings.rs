use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramSettings {
    pub programs: HashMap<String, String>,
    pub startup_programs: HashMap<String, String>,
}

impl ProgramSettings {
    pub fn new() -> Self {
        Self::default()
    }
}