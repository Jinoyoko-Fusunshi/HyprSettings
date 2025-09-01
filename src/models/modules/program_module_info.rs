use crate::models::modules::ProgramModuleCategory;

#[derive(Clone)]
pub struct ProgramModuleInfo {
    pub name: String,
    pub link: String,
    pub category: ProgramModuleCategory
}