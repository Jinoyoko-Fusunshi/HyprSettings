use crate::providers::application_provider::ApplicationProvider;
use crate::providers::module_provider::{
    FILE_MANAGER_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};

#[derive(Clone)]
pub struct GeneralSettingsState {
    pub terminal_path: Option<String>,
    pub file_manager_path: Option<String>,
    pub quick_search_path: Option<String>,
    pub notification_handler_path: Option<String>,
}

impl From<&ApplicationProvider> for GeneralSettingsState {
    fn from(value: &ApplicationProvider) -> Self {
        let program_provider = value.get_program_provider();
        let program_provider_ref = program_provider.borrow();

        let terminal_path = program_provider_ref.get_program(VIRTUAL_TERMINAL_ENTRY);
        let file_manager_path = program_provider_ref.get_program(FILE_MANAGER_ENTRY);
        let quick_search_path = program_provider_ref.get_program(QUICK_SEARCH_ENTRY);
        let notification_handler_path = program_provider_ref.get_program(NOTIFICATION_HANDLER_ENTRY);

        Self {
            terminal_path,
            file_manager_path,
            quick_search_path,
            notification_handler_path,
        }
    }
}