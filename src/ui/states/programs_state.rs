use crate::providers::application_provider::ApplicationProvider;
use crate::providers::module_provider::{
    FILE_MANAGER_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};

#[derive(Clone, Default)]
pub struct ProgramsState {
    pub terminal_path: Option<String>,
    pub file_manager_path: Option<String>,
    pub quick_search_path: Option<String>,
    pub notification_handler_path: Option<String>,
}

impl From<&ApplicationProvider> for ProgramsState {
    fn from(value: &ApplicationProvider) -> Self {
        let program_provider = value.get_program_provider();
        let program_provider_ref = program_provider.borrow();

        let terminal_path = program_provider_ref.get_program_path(VIRTUAL_TERMINAL_ENTRY.to_string());
        let file_manager_path = program_provider_ref.get_program_path(FILE_MANAGER_ENTRY.to_string());
        let quick_search_path = program_provider_ref.get_program_path(QUICK_SEARCH_ENTRY.to_string());
        let notification_handler_path = program_provider_ref.get_program_path(NOTIFICATION_HANDLER_ENTRY.to_string());

        Self {
            terminal_path,
            file_manager_path,
            quick_search_path,
            notification_handler_path,
        }
    }
}