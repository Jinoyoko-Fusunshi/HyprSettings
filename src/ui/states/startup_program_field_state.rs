#[derive(Clone, Default)]
pub struct StartupProgramFieldState {
    pub previous_program_name: String,
    pub program_name: String,
    pub program_path: String,
    pub programs: Vec<String>,
}