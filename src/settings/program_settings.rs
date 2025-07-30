use crate::settings::program::Program;

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