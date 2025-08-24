use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Entry, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, EditableExt, WidgetExt};
use crate::settings::settings_manager::SettingsManager;
use crate::ui::controls::input_field::{InputField, InputFieldState};
use crate::ui::states::general_settings_state::GeneralSettingsState;
use crate::ui::component::Component;
use crate::ui::updatable_component::UpdatableComponent;

pub struct GeneralSettings {
    settings_manager: Rc<RefCell<SettingsManager>>,
    general_box: gtk::Box,
    config_input_field: InputField,
    terminal_input_field: InputField,
    files_input_field: InputField,
    quick_search_input_field: InputField,
    lockscreen_input_field: InputField,
    notifications_input_field: InputField,
}

impl Component for GeneralSettings {
    fn init_events(&self) {
        let settings_manager = self.settings_manager.clone();
        let config_input_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_hyprland_config_program_path(input.text().to_string());
        };
        self.config_input_field.set_input_callback(config_input_change);

        let settings_manager = self.settings_manager.clone();
        let terminal_input_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_terminal_program_path(input.text().to_string());
        };
        self.terminal_input_field.set_input_callback(terminal_input_change);

        let settings_manager = self.settings_manager.clone();
        let files_input_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_files_program_path(input.text().to_string());
        };
        self.files_input_field.set_input_callback(files_input_change);

        let settings_manager = self.settings_manager.clone();
        let quick_search_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_quick_search_program_path(input.text().to_string());
        };
        self.quick_search_input_field.set_input_callback(quick_search_change);

        let settings_manager = self.settings_manager.clone();
        let lockscreen_input_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_lockscreen_program_path(input.text().to_string());
        };
        self.lockscreen_input_field.set_input_callback(lockscreen_input_change);

        let settings_manager = self.settings_manager.clone();
        let notifications_input_change = move |input: &Entry| {
            settings_manager.borrow_mut().set_notifications_program_path(input.text().to_string());
        };
        self.notifications_input_field.set_input_callback(notifications_input_change);
    }

    fn get_widget(&self) -> &gtk::Box {
        &self.general_box
    }
}

impl UpdatableComponent<GeneralSettingsState> for GeneralSettings {
    fn update_ui(&mut self, state: GeneralSettingsState) {
        let input_field_state = InputFieldState {
            label_text: "Hyprland config path:".to_string(),
            entry_text: state.hyprland_config_path,
            placeholder_text: "default: ~/.config/hypr/hyprland.cfg".to_string(),
        };
        self.config_input_field.update_ui(input_field_state);

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
            label_text: "Lock screen program path:".to_string(),
            entry_text: state.lock_screen_path,
            placeholder_text: "e.g. /usr/bin/hyprlock".to_string(),
        };
        self.lockscreen_input_field.update_ui(input_field_state);

        let input_field_state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: state.notification_handler_path,
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        self.notifications_input_field.update_ui(input_field_state);
    }
}

impl GeneralSettings {
    pub fn new(settings_manager: Rc<RefCell<SettingsManager>>) -> Self {
        const PROGRAMS_LABEL: &str = "Programs";
        
        let general_box = gtk::Box::new(Orientation::Vertical, 10);
        general_box.set_margin_top(10);
        general_box.set_margin_bottom(10);
        general_box.set_margin_start(10);
        general_box.set_margin_end(10);

        let programs_label = Label::new(Some(PROGRAMS_LABEL));
        let separator = Separator::new(Orientation::Horizontal);
        let config_input_field = InputField::new();

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

        let mut lockscreen_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Lock screen program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/hyprlock".to_string(),
        };
        lockscreen_input_field.update_ui(state);

        let mut notifications_input_field = InputField::new();
        let state = InputFieldState {
            label_text: "Notification handler program path:".to_string(),
            entry_text: None,
            placeholder_text: "e.g. /usr/bin/dryrun".to_string(),
        };
        notifications_input_field.update_ui(state);

        general_box.append(&programs_label);
        general_box.append(&separator);
        general_box.append(config_input_field.get_widget());
        general_box.append(terminal_input_field.get_widget());
        general_box.append(files_input_field.get_widget());
        general_box.append(quick_search_input_field.get_widget());
        general_box.append(lockscreen_input_field.get_widget());
        general_box.append(notifications_input_field.get_widget());

        Self {
            settings_manager,
            general_box,
            config_input_field,
            terminal_input_field,
            files_input_field,
            quick_search_input_field,
            lockscreen_input_field,
            notifications_input_field,
        }
    }
}