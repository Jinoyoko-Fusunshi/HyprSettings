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