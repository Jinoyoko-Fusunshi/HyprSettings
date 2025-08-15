use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Button, ComboBoxText, Entry, Orientation};
use gtk::prelude::{BoxExt, ButtonExt, ComboBoxExt, ComboBoxExtManual, EditableExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::ui::css_styles::CSSStyles;
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::editable_control::EditableControl;

#[derive(Clone)]
struct StartupProgramEntryRowModel {
    previous_selected_program_name: String,
    selected_program_name: String,
    selected_program_path: String,
    available_programs: Vec<String>,
}

#[derive(Clone)]
pub struct StartupProgramEntryRow {
    startup_entry_box: gtk::Box,
    delete_button: Button,
    program_name_input: Entry,
    program_path_input: Entry,
    selection_box: ComboBoxText,
    model: Rc<RefCell<StartupProgramEntryRowModel>>,
}

impl Panel for StartupProgramEntryRow {
    fn reload_settings(&self, settings: &Rc<RefCell<HyprlandSettings>>) {
        let programs = settings.borrow().programs.keys().cloned().collect();
        self.update_program_items_with_selection(programs);
    }

    fn get_container_box(&self) -> &gtk::Box {
        &self.startup_entry_box
    }
}

impl EditableControl for StartupProgramEntryRow {
    fn enable_control(&self) {
        self.delete_button.set_sensitive(false);
        self.program_name_input.set_sensitive(true);
        self.program_path_input.set_sensitive(true);
        self.selection_box.set_sensitive(true);
    }

    fn disable_control(&self) {
        self.delete_button.set_sensitive(true);
        self.program_name_input.set_sensitive(false);
        self.program_path_input.set_sensitive(false);
        self.selection_box.set_sensitive(false);
    }
}

impl StartupProgramEntryRow {
    pub fn new(
        settings: Rc<RefCell<HyprlandSettings>>,
        startup_program: Option<String>, available_programs: Vec<String>
    ) -> StartupProgramEntryRow {
        let mut program_path: Option<String> = None;
        if let Some(program_name) = startup_program.clone() {
            program_path = settings.borrow().startup_programs.get(&program_name).cloned();
        }

        let model = Rc::new(RefCell::new(StartupProgramEntryRowModel {
            previous_selected_program_name: startup_program.clone().unwrap_or(String::new()),
            selected_program_name: startup_program.clone().unwrap_or(String::new()),
            selected_program_path: program_path.clone().unwrap_or(String::new()),
            available_programs: available_programs.clone()
        }));

        let startup_entry_box = gtk::Box::new(Orientation::Horizontal, 10);
        let delete_button = Button::with_label("❌");
        delete_button.add_css_class(CSSStyles::CANCEL_STARTUP_PROGRAM_BUTTON);

        let program_name_input = Entry::new();
        if let Some(name) = startup_program.clone() {
            program_name_input.set_text(name.as_str());
        }

        let model_clone = model.clone();
        let program_name_input_changed_callback = move |entry: &Entry| {
            model_clone.borrow_mut().selected_program_name = entry.text().to_string()
        };
        program_name_input.connect_changed(program_name_input_changed_callback);

        let program_path_input = Entry::new();
        if let Some(path) = program_path {
            program_path_input.set_text(path.as_str());
        }

        let model_clone = model.clone();
        let program_path_input_changed_callback = move |entry: &Entry| {
            model_clone.borrow_mut().selected_program_path = entry.text().to_string()
        };
        program_path_input.connect_changed(program_path_input_changed_callback);

        let selection_box = ComboBoxText::new();
        let selection_box_changed_callback = Self::create_selection_box_changed_callback(
            settings.clone(), model.clone(), program_name_input.clone(), program_path_input.clone()
        );

        Self::add_program_items_with_selection(selection_box.clone(), model.clone(), startup_program, available_programs);

        selection_box.connect_changed(selection_box_changed_callback);

        startup_entry_box.append(&delete_button);
        startup_entry_box.append(&selection_box);
        startup_entry_box.append(&program_name_input);
        startup_entry_box.append(&program_path_input);

        Self {
            startup_entry_box,
            delete_button,
            program_name_input,
            program_path_input,
            selection_box,
            model
        }
    }

    fn create_selection_box_changed_callback(
        settings: Rc<RefCell<HyprlandSettings>>,
        startup_program_entry_row_model: Rc<RefCell<StartupProgramEntryRowModel>>,
        program_name_input: Entry, program_path_input: Entry
    ) -> impl Fn(&ComboBoxText) {
        let selection_box_changed_callback = move |combobox :&ComboBoxText| {
            if let Some(active_text) = combobox.active_text() {
                let mut startup_program_entry_row_model_mut = startup_program_entry_row_model.borrow_mut();
                let previous_selected_program = startup_program_entry_row_model_mut.selected_program_name.clone();
                startup_program_entry_row_model_mut.previous_selected_program_name = previous_selected_program.clone();
                drop(startup_program_entry_row_model_mut);

                let selected_text = active_text.to_string();
                if let Some(program_command) = settings.borrow_mut().programs.get(&selected_text.to_string())
                {
                    program_name_input.set_text(&selected_text);
                    program_path_input.set_text(program_command);
                }

                if selected_text == "Custom" {
                    program_name_input.set_editable(true);
                    program_name_input.remove_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);

                    program_path_input.set_editable(true);
                    program_path_input.remove_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);

                } else {
                    program_name_input.set_editable(false);
                    program_name_input.add_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);

                    program_path_input.set_editable(false);
                    program_path_input.add_css_class(CSSStyles::DISABLED_STARTUP_PROGRAM_BUTTON);
                }
            }
        };
        selection_box_changed_callback
    }

    pub fn set_deletion_click_callback(&self, delete_button_click_callback: impl Fn(&Button) + 'static) {
        self.delete_button.connect_clicked(delete_button_click_callback);
    }

    pub fn update_program_items_with_selection(&self, available_programs: Vec<String>) {
        self.selection_box.remove_all();

        let this = self.clone();
        let selected_program_name = this.model.borrow().selected_program_name.clone();
        Self::add_program_items_with_selection(
            this.selection_box, this.model.clone(), Some(selected_program_name),
            available_programs.clone()
        );
    }

    fn add_program_items_with_selection(
        selection_box: ComboBoxText, model: Rc<RefCell<StartupProgramEntryRowModel>>,
        program_name: Option<String>, available_programs: Vec<String>
    ) {
        let mut index = 0;
        let mut active_index = Some(index);

        selection_box.append_text("Custom");
        index = 1;

        for available_program in available_programs.iter() {
            selection_box.append_text(available_program);

            if let Some(program_name) = program_name.clone() {
                if available_program.clone() == program_name {
                    active_index = Some(index);
                }
            }

            index += 1;
        }

        model.borrow_mut().available_programs = available_programs.clone();

        if let Some(name) = program_name.clone() {
            model.borrow_mut().selected_program_name = name.clone();
        }

        selection_box.set_active(active_index);
    }

    pub fn save_setting(&self, settings: Rc<RefCell<HyprlandSettings>>) {
        let mut model_mut = self.model.borrow_mut();
        let program_name = model_mut.selected_program_name.clone();
        let program_path = model_mut.selected_program_path.clone();

        let mut settings_mut = settings.borrow_mut();
        settings_mut.startup_programs.remove(&model_mut.previous_selected_program_name);
        settings_mut.startup_programs.insert(program_name.clone(), program_path.clone());
        model_mut.previous_selected_program_name = program_name.clone();
    }

    pub fn remove_setting(&self, settings: Rc<RefCell<HyprlandSettings>>) {
        let model_ref = self.model.borrow();
        let program_name = model_ref.selected_program_name.clone();
        settings.borrow_mut().startup_programs.remove(&program_name);
    }
}