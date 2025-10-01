use gtk::{Button, ComboBoxText, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::css_styles::CSSStyles;
use crate::ui::controls::Control;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::selection_box::SelectionBox;
use crate::ui::manager::startup_program_field_manager::{StartupProgramFieldEvent, StartupProgramFieldManager};
use crate::ui::pages::keybinds::CUSTOM_ITEM;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::selection_box_state::SelectionBoxState;
use crate::ui::states::startup_program_field_state::StartupProgramFieldState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::{new_rc_mut, RcMut};

pub struct StartupProgramField {
    application_provider: ApplicationProvider,
    state: RcMut<StartupProgramFieldState>,
    startup_entry_box: GTKBox,
    delete_button: Button,
    program_name_input: Entry,
    program_path_input: Entry,
    selection_box: SelectionBox,
}

impl Control for StartupProgramField {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.startup_entry_box
    }
}

impl UpdatableControl<StartupProgramFieldState> for StartupProgramField {
    fn update_state(&mut self, state: StartupProgramFieldState) {
        let program = if state.program_name == CUSTOM_ITEM {
            None
        } else {
            Some(state.program_name.clone())
        };

        self.change_input_access(program.clone());
        self.set_program_fields(program.clone());
        *self.state.borrow_mut() = state.clone();
        
        let state = SelectionBoxState {
            label_text: "Program: ".to_string(),
            options: state.programs,
            selected_option: program
        };
        self.selection_box.update_state(state.clone());
    }

    fn get_current_state(&self) -> StartupProgramFieldState {
        self.state.borrow().clone()
    }
}

impl StateSavableControl for StartupProgramField {
    fn save_settings(&self, application_provider: ApplicationProvider) {
        let mut state_mut = self.state.borrow_mut();
        let previous_program_name = state_mut.previous_program_name.clone();
        let program_name = state_mut.program_name.clone();
        let program_path = state_mut.program_path.clone();

        let program_provider = application_provider.get_program_provider();
        let mut program_provider_mut = program_provider.borrow_mut();
        program_provider_mut.remove_program(previous_program_name.clone());
        program_provider_mut.add_program(program_name.clone(), program_path.clone());

        program_provider_mut.remove_startup_program(previous_program_name.clone());
        program_provider_mut.add_startup_program(program_name.clone(), program_path.clone());
        state_mut.previous_program_name = program_name.clone();
    }

    fn remove_settings(&self, application_provider: ApplicationProvider) {
        let state_ref = self.state.borrow();
        let program_name = state_ref.program_name.clone();

        let program_provider = application_provider.get_program_provider();
        let mut program_provider_mut = program_provider.borrow_mut();
        program_provider_mut.remove_program(program_name.clone());
        program_provider_mut.remove_startup_program(program_name.clone());
    }
}

impl ActivableControl for StartupProgramField {
    fn enable_control(&self) {
        self.delete_button.set_sensitive(false);
        self.program_name_input.set_sensitive(true);
        self.program_path_input.set_sensitive(true);
        self.selection_box.enable_control();
    }

    fn disable_control(&self) {
        self.delete_button.set_sensitive(true);
        self.program_name_input.set_sensitive(false);
        self.program_path_input.set_sensitive(false);
        self.selection_box.disable_control();
    }
}

impl StartupProgramField {
    pub fn new(application_provider: ApplicationProvider) -> StartupProgramField {
        let state = new_rc_mut(StartupProgramFieldState {
            previous_program_name: "".to_string(),
            program_path: String::new(),
            program_name: "".to_string(),
            programs: vec![CUSTOM_ITEM.to_string()],
        });

        let startup_entry_box = BoxBuilder::new("startup-entry")
            .set_orientation(Orientation::Horizontal)
            .build();

        let delete_button = Button::with_label("❌");

        let program_name_input = Entry::new();
        let program_path_input = Entry::new();
        let selection_box = SelectionBox::new();

        startup_entry_box.append(&delete_button);
        startup_entry_box.append(selection_box.get_widget());
        startup_entry_box.append(&program_name_input);
        startup_entry_box.append(&program_path_input);

        Self {
            application_provider,
            state,
            startup_entry_box,
            delete_button,
            program_name_input,
            program_path_input,
            selection_box,
        }
    }

    pub fn init_events(&self, startup_program_field_manager: StartupProgramFieldManager) {
        let state = self.state.clone();
        let program_name_input_change = move |entry: &Entry| {
            state.borrow_mut().program_name = entry.text().to_string()
        };
        self.program_name_input.connect_changed(program_name_input_change);

        let state = self.state.clone();
        let state_clone = state.clone();
        let program_path_input_change = move |entry: &Entry| {
            state_clone.borrow_mut().program_path = entry.text().to_string()
        };
        self.program_path_input.connect_changed(program_path_input_change);

        let selection_box_change = Self::create_selection_box_change(
            startup_program_field_manager
        );
        self.selection_box.set_selection_change(selection_box_change);
    }

    pub fn set_program_fields(&self, selected_program_name: Option<String>) {
        let program_provider = self.application_provider.get_program_provider();
        let name = match selected_program_name.clone() {
            Some(path) => match program_provider.borrow().get_program_path_or_module(path.clone()) {
                Some(_) => path,
                None => "".to_string(),
            }
            None => "".to_string(),
        };

        let path = match selected_program_name.clone() {
            Some(program) => program_provider.borrow()
                .get_program_path_or_module(program)
                .unwrap_or_else(|| "".to_string()),
            None => "".to_string(),
        };

        self.program_name_input.set_text(name.as_str());
        self.program_path_input.set_text(path.as_str());
    }

    pub fn change_input_access(&self, selected_program: Option<String>) {
        if let Some(_) = selected_program {
            self.program_name_input.set_editable(false);
            self.program_name_input.add_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);

            self.program_path_input.set_editable(false);
            self.program_path_input.add_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);
        } else {
            self.program_name_input.set_editable(true);
            self.program_name_input.remove_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);

            self.program_path_input.set_editable(true);
            self.program_path_input.remove_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);
        }
    }

    fn create_selection_box_change(startup_program_field_manager: StartupProgramFieldManager) -> impl Fn(&ComboBoxText) {
        let selection_box_changed_callback = move |combobox :&ComboBoxText| {
            if let Some(active_text) = combobox.active_text() {
                let selected_text = active_text.to_string();
                let program= if selected_text == CUSTOM_ITEM {
                    None
                } else {
                    Some(selected_text)
                };

                startup_program_field_manager.send_event(StartupProgramFieldEvent::SelectionChanged(program));
            }
        };
        selection_box_changed_callback
    }

    pub fn set_deletion_click_callback(&self, delete_button_click_callback: impl Fn(&Button) + 'static) {
        self.delete_button.connect_clicked(delete_button_click_callback);
    }
}