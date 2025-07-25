#[derive(Debug)]
pub struct ProgramSettings {
    pub hypr_land_config: Program,
    pub virtual_terminal: Program,
    pub file_manager: Program,
    pub quick_search: Program,
    pub lock_screen: Program,
    pub notification_handler: Program,
}

impl ProgramSettings {
    pub fn new() -> ProgramSettings {
        Self {
            hypr_land_config: Program::new(String::new(), String::new()),
            virtual_terminal: Program::new(String::new(), String::new()),
            file_manager: Program::new(String::new(), String::new()),
            quick_search: Program::new(String::new(), String::new()),
            lock_screen: Program::new(String::new(), String::new()),
            notification_handler: Program::new(String::new(), String::new()),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub display_name: String,
    pub program_path: String,
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Self {
            display_name: self.display_name.clone(),
            program_path: self.program_path.clone()
        }
    }
}

impl Program {
    pub fn new(display_name: String, program_path: String) -> Program {
        Self {
            display_name,
            program_path,
        }
    }
}