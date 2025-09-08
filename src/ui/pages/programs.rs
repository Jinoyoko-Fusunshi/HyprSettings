use gtk::Entry;
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::module_provider::{
    FILE_MANAGER_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};
use crate::types::GTKBox;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::input_field::InputField;
use crate::ui::states::general_settings_state::GeneralSettingsState;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::updatable_control::UpdatableControl;

pub struct Programs {
    application_provider: ApplicationProvider,
    general_box: GTKBox,
    terminal_input_field: InputField,
    files_input_field: InputField,
    quick_search_input_field: InputField,
    notifications_input_field: InputField,
}

impl Control for Programs {
    fn init_events(&self) {
        let program_provider =  self.application_provider.get_program_provider();
        let terminal_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                VIRTUAL_TERMINAL_ENTRY.to_string(), input.text().to_string()
            );
        };
        self.terminal_input_field.set_input_callback(terminal_input_change);

        let program_provider = self.application_provider.get_program_provider();
        let files_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                FILE_MANAGER_ENTRY.to_string(), input.text().to_string()
            );
        };
        self.files_input_field.set_input_callback(files_input_change);

        let program_provider =  self.application_provider.get_program_provider();
        let quick_search_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                QUICK_SEARCH_ENTRY.to_string(), input.text().to_string()
            );
        };
        self.quick_search_input_field.set_input_callback(quick_search_change);

        let program_provider =  self.application_provider.get_program_provider();
        let notifications_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                NOTIFICATION_HANDLER_ENTRY.to_string(), input.text().to_string()
            );
        };
        self.notifications_input_field.set_input_callback(notifications_input_change);
    }

    fn get_widget(&self) -> &GTKBox {
        &self.general_box
    }
}

impl UpdatableControl<GeneralSettingsState> for Programs {
    fn update_ui(&mut self, state: GeneralSettingsState) {
        let input_field_state = InputFieldState {
            label_text: "Virtual terminal program path:".to_string(),
            entry_text: state.terminal_path,
            placeholder_text: "e.g. /usr/bin/alacritty".to_string(),
        };
        self.terminal_input_field.update_ui(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "File manager program path:".to_string(),
            entry_text: state.file_manager_path,
            placeholder_text: "e.g. /usr/bin/nautilus".to_string(),
        };
        self.files_input_field.update_ui(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Quick search program path:".to_string(),
            entry_text: state.quick_search_path,
            placeholder_text: "e.g. /usr/bin/anyrun".to_string(),
        };
        self.quick_search_input_field.update_ui(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: state.notification_handler_path,
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        self.notifications_input_field.update_ui(input_field_state);
    }
}

impl Programs {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        const PROGRAMS_LABEL: &str = "Programs";
        
        let general_box = SectionBoxBuilder::new("programs", DEFAULT_MARGIN)
            .create_header_elements(PROGRAMS_LABEL)
            .build().expect("Failed to create general box");
        general_box.set_margin_top(10);
        general_box.set_margin_bottom(10);
        general_box.set_margin_start(10);
        general_box.set_margin_end(10);

        let mut terminal_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Virtual terminal program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/alacritty".to_string(),
        };
        terminal_input_field.update_ui(state);

        let mut files_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "File manager program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/nautilus".to_string(),
        };
        files_input_field.update_ui(state);

        let mut quick_search_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Quick search program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/anyrun".to_string(),
        };
        quick_search_input_field.update_ui(state);

        let mut notifications_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        notifications_input_field.update_ui(state);

        general_box.append(terminal_input_field.get_widget());
        general_box.append(files_input_field.get_widget());
        general_box.append(quick_search_input_field.get_widget());
        general_box.append(notifications_input_field.get_widget());

        Self {
            application_provider,
            general_box,
            terminal_input_field,
            files_input_field,
            quick_search_input_field,
            notifications_input_field,
        }
    }
}