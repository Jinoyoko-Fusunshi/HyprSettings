use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button, Label, Orientation, Separator};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::ui::css_styles::CSSStyles;
use crate::settings::hyprland_settings::HyprlandSettings;
use startup_program_entry_row::StartupProgramEntryRow;

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
        let settings_clone = settings.clone();
        let startup_entries_box_clone = startup_entries_box.clone();
        let startup_entries_copy = startup_entries.clone();

        let create_button_click_callback = move |_ :&Button| {
            let startup_entry = StartupProgramEntryRow::new(&startup_entries_box_clone, &settings_clone);
            startup_entries_box_clone.append(startup_entry.get_container_box());
            startup_entries_copy.borrow_mut().push(startup_entry);
        };
        let create_button = Button::with_label("➕ Add startup program");
        create_button.set_hexpand(false);
        create_button.set_halign(Align::Start);
        create_button.add_css_class(CSSStyles::CREATE_STARTUP_PROGRAM_BUTTON);
        create_button.connect_clicked(create_button_click_callback);

        program_panel_box.append(&startup_programs_label);
        program_panel_box.append(&separator);
        program_panel_box.append(&startup_entries_box);
        program_panel_box.append(&create_button);

        Self {
            startup_entries,
            program_panel_box,
        }
    }
}