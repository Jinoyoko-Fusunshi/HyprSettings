use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Orientation, ScrolledWindow};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::providers::application_provider::ApplicationProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::css_styles::CSSStyles;
use crate::ui::controls::startup_program_field::StartupProgramField;
use crate::ui::controls::Control;
use crate::ui::controls::editable_control::{EditableControl};
use crate::ui::managed_control::ManagedControl;
use crate::ui::manager::editable_control_manager::EditableControlManager;
use crate::ui::manager::startup_program_field_manager::StartupProgramFieldManager;
use crate::ui::pages::keybinds::CUSTOM_ITEM;
use crate::ui::section_box_builder::SectionBoxBuilder;
use crate::ui::state_savable_control::StateSavableControl;
use crate::ui::states::editable_control_state::{EditMode, EditableControlState};
use crate::ui::states::startup_program_field_state::StartupProgramFieldState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::new_rc_mut;

pub struct StartupPrograms {
    startup_programs_scroll_box: GTKBox,
    startup_programs_entries_box: GTKBox,
    create_button: Button,
}

impl Control for StartupPrograms {
    fn get_widget(&self) -> &GTKBox {
        &self.startup_programs_scroll_box
    }
}

impl StartupPrograms {
    pub fn new() -> Self {
        const STARTUP_PROGRAMS_LABEL: &str = "Programs on system start";

        let startup_programs_box = SectionBoxBuilder::new("startup-programs", DEFAULT_MARGIN)
            .create_header_elements(STARTUP_PROGRAMS_LABEL)
            .build().expect("Failed to create startup programs box");

        let startup_programs_scroll_window = ScrolledWindow::new();
        startup_programs_scroll_window.set_widget_name("startup-programs-scroll-window");
        startup_programs_scroll_window.set_vexpand(true);
        startup_programs_scroll_window.set_child(Some(&startup_programs_box));

        let startup_programs_scroll_box = BoxBuilder::new("startup-programs-scroll-box")
            .set_orientation(Orientation::Vertical)
            .build();
        startup_programs_scroll_box.append(&startup_programs_scroll_window);

        let startup_program_entries_box = BoxBuilder::new(".startup-program-entries")
            .set_orientation(Orientation::Vertical)
            .build();

        let create_button = Button::with_label("➕ Add startup program");
        create_button.set_hexpand(false);
        create_button.set_halign(Align::Start);
        create_button.add_css_class(CSSStyles::CREATE_STARTUP_PROGRAM_BUTTON);

        startup_programs_box.append(&startup_program_entries_box);
        startup_programs_box.append(&create_button);

        Self {
            startup_programs_scroll_box,
            startup_programs_entries_box: startup_program_entries_box,
            create_button
        }
    }

    pub fn init_ui(&self, application_provider: ApplicationProvider) {
        let program_provider = application_provider.get_program_provider();

        let mut programs = vec![CUSTOM_ITEM.to_string()];
        programs.append(&mut program_provider.borrow().get_program_and_module_names());

        for (program_name, program_path) in program_provider.borrow().get_startup_programs() {
            let startup_program_field = Self::create_editable_startup_program_field(
                application_provider.clone(), self.startup_programs_entries_box.clone(),  program_name,
                program_path, programs.clone(), EditMode::Locked
            );
            self.startup_programs_entries_box.append(startup_program_field.borrow().get_widget());
        }
    }

    pub fn init_events(&self, application_provider: ApplicationProvider) {
        let program_provider = application_provider.get_program_provider();

        let mut programs = vec![CUSTOM_ITEM.to_string()];
        programs.append(&mut program_provider.borrow().get_program_and_module_names());

        let startup_program_entries_box = self.startup_programs_entries_box.clone();
        let create_startup_program_button_click = move |_ :&Button| {
            let editable_control = Self::create_editable_startup_program_field(
                application_provider.clone(),
                startup_program_entries_box.clone(),
                CUSTOM_ITEM.to_string(),
                "".to_string(),
                programs.clone(),
                EditMode::Edit
            );

            startup_program_entries_box.append(editable_control.borrow().get_widget());

        };
        self.create_button.connect_clicked(create_startup_program_button_click);
    }

    fn create_editable_startup_program_field(
        application_provider: ApplicationProvider, startup_program_entries_box: GTKBox, program_name: String, program_path: String,
        programs: Vec<String>, edit_mode: EditMode,
    ) -> Rc<RefCell<EditableControl<StartupProgramField, StartupProgramFieldState>>>{
        let state = StartupProgramFieldState {
            previous_program_name: program_name.clone(),
            program_name,
            program_path,
            programs,
        };

        let startup_program_field = new_rc_mut(StartupProgramField::new(application_provider.clone()));
        let startup_program_field_manager = StartupProgramFieldManager::new(startup_program_field.clone());
        startup_program_field.borrow_mut().update_state(state.clone());
        startup_program_field.borrow().init_events(startup_program_field_manager);

        let editable_control_state = EditableControlState {
            edit_mode
        };
        let editable_control = new_rc_mut(EditableControl::new(
            startup_program_field.clone()
        ));
        editable_control.borrow_mut().update_state(editable_control_state.clone());

        let editable_control_manager = EditableControlManager::new(
            editable_control.clone(), application_provider.clone()
        );
        editable_control.borrow_mut().init_events_by_manager(editable_control_manager);

        let editable_control_clone = editable_control.clone();
        let startup_program_field_clone = startup_program_field.clone();

        let startup_program_field_delete_click = move |_: &Button| {
            startup_program_entries_box.remove(editable_control_clone.borrow().get_widget());
            startup_program_field_clone.borrow().remove_settings(application_provider.clone());
        };
        startup_program_field.borrow().set_deletion_click_callback(startup_program_field_delete_click);
        editable_control
    }
}