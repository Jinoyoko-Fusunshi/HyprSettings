use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::ui::css_styles::CSSStyles;
use crate::settings::hyprland_settings::HyprlandSettings;
use startup_program_entry_row::StartupProgramEntryRow;
use crate::ui::controls::editable_control_element::{EditMode, EditableControlElement};

mod startup_program_entry_row;

pub struct StartupProgramsPanel {
    program_panel_box: gtk::Box,
    startup_entries: Rc<RefCell<Vec<StartupProgramEntryRow>>>,
}

impl Panel for StartupProgramsPanel {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {
        for entry in self.startup_entries.borrow().iter() {
            entry.reload_settings(settings);
        }
    }

    fn get_container_box(&self) -> &gtk::Box {
        &self.program_panel_box
    }
}

impl Clone for StartupProgramsPanel {
    fn clone(&self) -> Self {
        Self {
            program_panel_box: self.program_panel_box.clone(),
            startup_entries: self.startup_entries.clone(),
        }
    }
}

impl StartupProgramsPanel {
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        const PROGRAMS_ON_STARTUP_LABEL: &str = "Programs on system start";

        let program_panel_box = gtk::Box::new(Orientation::Vertical, 10);
        program_panel_box.set_margin_top(10);
        program_panel_box.set_margin_bottom(10);
        program_panel_box.set_margin_start(10);
        program_panel_box.set_margin_end(10);

        let startup_programs_label = Label::new(Some(PROGRAMS_ON_STARTUP_LABEL));
        let separator = Separator::new(Orientation::Horizontal);

        let startup_entries_box = gtk::Box::new(Orientation::Vertical, 10);
        let startup_entries = Rc::new(RefCell::new(vec![]));

        let create_button = Button::with_label("➕ Add startup program");
        create_button.set_hexpand(false);
        create_button.set_halign(Align::Start);
        create_button.add_css_class(CSSStyles::CREATE_STARTUP_PROGRAM_BUTTON);

        let settings_clone = settings.clone();
        let startup_entries_box_clone = startup_entries_box.clone();
        let startup_entries_copy = startup_entries.clone();

        let create_startup_program_button_click_callback = move |_ :&Button| {
            Self::create_startup_program_entry(
                settings_clone.clone(),
                startup_entries_box_clone.clone(),
                startup_entries_copy.clone(),
                None, EditMode::Edit
            );
        };
        create_button.connect_clicked(create_startup_program_button_click_callback);

        Self::create_startup_programs_from_settings(
            &settings, startup_entries_box.clone(), startup_entries.clone()
        );

        program_panel_box.append(&startup_programs_label);
        program_panel_box.append(&separator);
        program_panel_box.append(&startup_entries_box);
        program_panel_box.append(&create_button);

        Self {
            program_panel_box,
            startup_entries,
        }
    }

    fn create_startup_programs_from_settings(
        settings: &Rc<RefCell<HyprlandSettings>>,
        startup_entries_box: gtk::Box,
        startup_entries: Rc<RefCell<Vec<StartupProgramEntryRow>>>
    ) {
        let startup_programs: Vec<String> = settings.borrow().startup_programs.clone()
            .keys()
            .map(|program_name| program_name.clone())
            .collect();

        for startup_program in startup_programs.clone() {
            Self::create_startup_program_entry(
                settings.clone(),
                startup_entries_box.clone(),
                startup_entries.clone(),
                Some(startup_program),
                EditMode::Locked
            );
        }
    }

    fn create_startup_program_entry(
        settings: Rc<RefCell<HyprlandSettings>>, startup_entries_box: gtk::Box,
        startup_entries: Rc<RefCell<Vec<StartupProgramEntryRow>>>,
        startup_program: Option<String>, edit_mode: EditMode
    ) {
        let available_programs: Vec<String> = settings.borrow().programs.clone()
            .keys()
            .map(|program_name| program_name.clone())
            .collect();

        let startup_entries_box_clone = startup_entries_box.clone();
        let startup_entry_row = StartupProgramEntryRow::new(
            settings.clone(), startup_program.clone(), available_programs
        );

        let editable_control_element = EditableControlElement::new(
            startup_entry_row.clone(), edit_mode
        );

        let startup_entry_row_clone = startup_entry_row.clone();
        let editable_control_element_button_click_callback = move |settings: Rc<RefCell<HyprlandSettings>>| {
            startup_entry_row_clone.save_setting(settings);
        };

        let settings_clone = settings.clone();
        editable_control_element.set_toggle_button_click_callback(
            settings_clone, editable_control_element_button_click_callback
        );

        let editable_control_element_clone = editable_control_element.clone();
        let startup_entry_row_clone = startup_entry_row.clone();
        let settings_clone = settings.clone();
        let startup_entry_delete_callback = move |_: &Button| {
            startup_entries_box_clone.remove(editable_control_element_clone.get_container_box());
            startup_entry_row_clone.remove_setting(settings_clone.clone());
        };
        startup_entry_row.set_deletion_click_callback(startup_entry_delete_callback);

        let startup_entries_box_clone = startup_entries_box.clone();
        startup_entries_box_clone.append(editable_control_element.get_container_box());
        startup_entries.borrow_mut().push(startup_entry_row);
    }
}