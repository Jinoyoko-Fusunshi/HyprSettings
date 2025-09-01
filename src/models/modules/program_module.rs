use crate::models::modules::program_module_info::ProgramModuleInfo;

#[derive(Clone)]
pub struct ProgramModule {
    pub info: ProgramModuleInfo,
    pub version: Option<String>,
}