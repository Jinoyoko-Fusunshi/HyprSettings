use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::ui::css_styles::CSSStyles;
use crate::ui::controls::startup_program_field::StartupProgramField;
use crate::ui::controls::Control;
use crate::ui::controls::editable_control_element::{EditMode, EditableControlElement, EditableControlElementManager};
use crate::ui::manager::startup_program_field_manager::StartupProgramFieldManager;
use crate::ui::pages::settings::keybinds_settings::CUSTOM_ITEM;
use crate::ui::statable_control::StatableControl;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::editable_control_element_state::EditableControlElementState;
use crate::ui::states::startup_program_field_state::StartupProgramFieldState;
use crate::ui::updatable_control::UpdatableControl;

pub struct StartupProgramsSettings {
    startup_program_box: gtk::Box,
    startup_program_entries_box: gtk::Box,
    create_button: Button,
}

impl Control for StartupProgramsSettings {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.startup_program_box
    }
}

impl StartupProgramsSettings {
    pub fn new() -> Self {
        const PROGRAMS_ON_STARTUP_LABEL: &str = "Programs on system start";

        let startup_program_box = gtk::Box::new(Orientation::Vertical, 10);
        startup_program_box.set_margin_top(10);
        startup_program_box.set_margin_bottom(10);
        startup_program_box.set_margin_start(10);
        startup_program_box.set_margin_end(10);

        let startup_programs_label = Label::new(Some(PROGRAMS_ON_STARTUP_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        let startup_program_entries_box = gtk::Box::new(Orientation::Vertical, 10);

        let create_button = Button::with_label("➕ Add startup program");
        create_button.set_hexpand(false);
        create_button.set_halign(Align::Start);
        create_button.add_css_class(CSSStyles::CREATE_STARTUP_PROGRAM_BUTTON);

        startup_program_box.append(&startup_programs_label);
        startup_program_box.append(&separator);
        startup_program_box.append(&startup_program_entries_box);
        startup_program_box.append(&create_button);

        Self {
            startup_program_box,
            startup_program_entries_box,
            create_button
        }
    }

    pub fn init_ui(&self, application_provider: ApplicationProvider) {
        let settings_provider = application_provider.get_settings_provider();

        let mut programs = vec![CUSTOM_ITEM.to_string()];
        programs.append(&mut settings_provider.borrow().get_program_names());

        for (program_name, program_path) in settings_provider.borrow().get_startup_programs() {
            let startup_program_field = Self::create_editable_startup_program_field(
                application_provider.clone(), self.startup_program_entries_box.clone(),  program_name,
                program_path, programs.clone(), EditMode::Locked
            );
            self.startup_program_entries_box.append(startup_program_field.borrow().get_widget());
        }
    }

    pub fn init_events(&self, application_provider: ApplicationProvider) {
        let settings_provider = application_provider.get_settings_provider();

        let mut programs = vec![CUSTOM_ITEM.to_string()];
        programs.append(&mut settings_provider.borrow().get_program_names());

        let startup_program_entries_box = self.startup_program_entries_box.clone();
        let create_startup_program_button_click = move |_ :&Button| {
            let editable_control_element = Self::create_editable_startup_program_field(
                application_provider.clone(),
                startup_program_entries_box.clone(),
                CUSTOM_ITEM.to_string(),
                "".to_string(),
                programs.clone(),
                EditMode::Edit
            );

            startup_program_entries_box.append(editable_control_element.borrow().get_widget());

        };
        self.create_button.connect_clicked(create_startup_program_button_click);
    }

    fn create_editable_startup_program_field(
        application_provider: ApplicationProvider, startup_program_entries_box: gtk::Box, program_name: String, program_path: String,
        programs: Vec<String>, edit_mode: EditMode,
    ) -> Rc<RefCell<EditableControlElement<StartupProgramField>>>{
        let state = StartupProgramFieldState {
            previous_program_name: program_name.clone(),
            program_name,
            program_path,
            programs,
        };

        let startup_program_field = Rc::new(RefCell::new(StartupProgramField::new(application_provider.clone())));
        let startup_program_field_manager = StartupProgramFieldManager::new(startup_program_field.clone());
        startup_program_field.borrow_mut().update_state(state.clone());
        startup_program_field.borrow_mut().update_ui(state.clone());
        startup_program_field.borrow().init_events(startup_program_field_manager);

        let editable_control_element_state = EditableControlElementState {
            edit_mode
        };
        let mut editable_control_element = EditableControlElement::new(
            startup_program_field.clone()
        );
        editable_control_element.update_state(editable_control_element_state.clone());
        editable_control_element.update_ui(editable_control_element_state.clone());

        let editable_control_element_rc = Rc::new(RefCell::new(editable_control_element));
        let editable_control_element_manager = EditableControlElementManager::new(
            editable_control_element_rc.clone(), application_provider.clone()
        );
        editable_control_element_rc.borrow_mut().init_events(editable_control_element_manager);

        let editable_control_element_rc_clone = editable_control_element_rc.clone();
        let startup_program_field_clone = startup_program_field.clone();

        let startup_program_field_delete_click = move |_: &Button| {
            startup_program_entries_box.remove(editable_control_element_rc_clone.borrow().get_widget());
            startup_program_field_clone.borrow().remove_settings(application_provider.clone());
        };
        startup_program_field.borrow().set_deletion_click_callback(startup_program_field_delete_click);
        editable_control_element_rc
    }
}