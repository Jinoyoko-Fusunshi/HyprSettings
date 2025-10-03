use gtk::Entry;
use gtk::prelude::{BoxExt, EditableExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::module_provider::{
    FILE_MANAGER_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY
};
use crate::types::GTKBox;
use crate::ui::boxes::{Boxes, DEFAULT_MARGIN};
use crate::ui::controls::input_field::InputField;
use crate::ui::states::programs_state::ProgramsState;
use crate::ui::controls::Control;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::updatable_control::UpdatableControl;

pub struct Programs {
    state: ProgramsState,
    general_box: GTKBox,
    terminal_input_field: InputField,
    files_input_field: InputField,
    quick_search_input_field: InputField,
    notifications_input_field: InputField,
}

impl Control for Programs {
    fn get_widget(&self) -> &GTKBox {
        &self.general_box
    }
}

impl UpdatableControl<ProgramsState> for Programs {
    fn update_state(&mut self, state: ProgramsState) {
        let input_field_state = InputFieldState {
            label_text: "Virtual terminal program path:".to_string(),
            entry_text: state.terminal_path.clone(),
            placeholder_text: "e.g. /usr/bin/alacritty".to_string(),
        };
        self.terminal_input_field.update_state(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "File manager program path:".to_string(),
            entry_text: state.file_manager_path.clone(),
            placeholder_text: "e.g. /usr/bin/nautilus".to_string(),
        };
        self.files_input_field.update_state(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Quick search program path:".to_string(),
            entry_text: state.quick_search_path.clone(),
            placeholder_text: "e.g. /usr/bin/anyrun".to_string(),
        };
        self.quick_search_input_field.update_state(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: state.notification_handler_path.clone(),
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        self.notifications_input_field.update_state(input_field_state);

        self.state = state;
    }

    fn get_current_state(&self) -> ProgramsState {
        self.state.clone()
    }
}

impl Programs {
    pub fn new(application_provider: ApplicationProvider) -> Self {
        const PROGRAMS_LABEL: &str = "Programs";
        
        let general_box = SectionBoxBuilder::new("programs", DEFAULT_MARGIN)
            .create_header_elements(PROGRAMS_LABEL)
            .build().expect("Failed to create general box");
        Boxes::set_margin(&general_box, DEFAULT_MARGIN);

        let mut terminal_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Virtual terminal program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/alacritty".to_string(),
        };
        terminal_input_field.update_state(state);

        let program_provider =  application_provider.get_program_provider();
        let terminal_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                VIRTUAL_TERMINAL_ENTRY.to_string(), input.text().to_string()
            );
        };
        terminal_input_field.set_input_callback(terminal_input_change);

        let mut files_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "File manager program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/nautilus".to_string(),
        };
        files_input_field.update_state(state);

        let program_provider = application_provider.get_program_provider();
        let files_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                FILE_MANAGER_ENTRY.to_string(), input.text().to_string()
            );
        };
        files_input_field.set_input_callback(files_input_change);

        let mut quick_search_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Quick search program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/anyrun".to_string(),
        };
        quick_search_input_field.update_state(state);

        let program_provider =  application_provider.get_program_provider();
        let quick_search_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                QUICK_SEARCH_ENTRY.to_string(), input.text().to_string()
            );
        };
        quick_search_input_field.set_input_callback(quick_search_change);

        let mut notifications_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        notifications_input_field.update_state(state);

        let program_provider =  application_provider.get_program_provider();
        let notifications_input_change = move |input: &Entry| {
            program_provider.borrow_mut().set_program_path(
                NOTIFICATION_HANDLER_ENTRY.to_string(), input.text().to_string()
            );
        };
        notifications_input_field.set_input_callback(notifications_input_change);

        general_box.append(terminal_input_field.get_widget());
        general_box.append(files_input_field.get_widget());
        general_box.append(quick_search_input_field.get_widget());
        general_box.append(notifications_input_field.get_widget());

        let state = Default::default();

        Self {
            state,
            general_box,
            terminal_input_field,
            files_input_field,
            quick_search_input_field,
            notifications_input_field,
        }
    }
}