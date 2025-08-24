#[derive(Clone)]
pub struct StartupProgramFieldState {
    pub previous_program_name: String,
    pub program_name: String,
    pub program_path: String,
    pub programs: Vec<String>,
}