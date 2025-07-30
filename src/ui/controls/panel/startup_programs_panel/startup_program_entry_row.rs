use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Button, ComboBoxText, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, ComboBoxExt, ComboBoxExtManual, EditableExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::ui::css_styles::CSSStyles;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct StartupProgramEntryRow {
    program_entry_box: gtk::Box,
    selection_box: ComboBoxText,
}

impl Panel for StartupProgramEntryRow {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {
        let programs = settings.borrow_mut().program_settings
            .iter()
            .map(|(program_name, _)| {
                program_name.clone()
            })
            .collect();
        self.update_selection_box(&programs);
    }

    fn get_container_box(&self) -> &gtk::Box {
        &self.program_entry_box
    }
}

impl StartupProgramEntryRow {
    pub fn new(
        startup_entries_box: &gtk::Box, settings: &Rc<RefCell<HyprlandSettings>>
    ) -> StartupProgramEntryRow {
        // Startup entry box
        let startup_entry_box = gtk::Box::new(Orientation::Horizontal, 10);

        // Delete startup entry button
        let startup_entry_box_clone = startup_entry_box.clone();
        let startup_entries_box_clone_clone = startup_entries_box.clone();
        let cancel_button_click_callback = move |_ :&Button| {
            startup_entries_box_clone_clone.remove(&startup_entry_box_clone);
        };
        let cancel_button = Button::with_label("❌");
        cancel_button.connect_clicked(cancel_button_click_callback);
        cancel_button.add_css_class(CSSStyles::CANCEL_STARTUP_PROGRAM_BUTTON);

        // Program input box
        let program_path_input = Entry::new();
        let program_path_input_clone = program_path_input.clone();

        // Program selection box
        let settings_clone = settings.clone();
        let selection_box = ComboBoxText::new();
        selection_box.append_text("Custom");
        selection_box.set_active(Some(0));

        for (program_name, _) in settings.borrow().program_settings.iter() {
            selection_box.append_text(program_name);
        }
        selection_box.connect_changed(move |combobox :&ComboBoxText| {
            if let Some(active_text) = combobox.active_text() {
                let selected_text = active_text.to_string();
                if let Some(program_command) = settings_clone.borrow_mut()
                    .program_settings.get(&selected_text.to_string()) {
                    program_path_input_clone.set_text(program_command);
                }

                if selected_text == "Custom" {
                    program_path_input_clone.set_editable(true);
                    program_path_input_clone.remove_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON)
                } else {
                    program_path_input_clone.set_editable(false);
                    program_path_input_clone.add_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);
                }
            }
        });

        startup_entry_box.append(&cancel_button);
        startup_entry_box.append(&selection_box);
        startup_entry_box.append(&program_path_input);

        Self {
            program_entry_box: startup_entry_box,
            selection_box,
        }
    }

    pub fn update_selection_box(&self, programs: &Vec<String>) {
        self.selection_box.remove_all();

        self.selection_box.append_text("Custom");
        for program in programs {
            self.selection_box.append_text(program);
        }

        self.selection_box.set_active(Some(0));
    }
}