use gtk::{Align, Entry, Label, Orientation};
use gtk::prelude::{BoxExt, EditableExt, EntryExt, WidgetExt};
use crate::ui::component::Component;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::updatable_component::UpdatableComponent;

pub struct InputFieldState {
    pub label_text: String,
    pub entry_text: Option<String>,
    pub placeholder_text: String,
}

pub struct InputField {
    input_field_box: gtk::Box,
    input_label: Label,
    input_entry: Entry
}

impl Component for InputField {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.input_field_box
    }
}

impl UpdatableComponent<InputFieldState> for InputField {
    fn update_ui(&mut self, state: InputFieldState) {
        self.input_label.set_text(state.label_text.as_str());

        if let Some(text) = state.entry_text {
            self.input_entry.set_text(text.as_str());
        } else {
            self.input_entry.set_placeholder_text(Some(state.placeholder_text.as_str()));
        }
    }
}

impl ActivableControl for InputField {
    fn enable_control(&self) {
        self.input_entry.set_sensitive(true);
    }

    fn disable_control(&self) {
        self.input_entry.set_sensitive(false);
    }
}

impl InputField {
    pub fn new () -> Self {
        let panel = gtk::Box::new(Orientation::Vertical, 0);
        panel.set_margin_bottom(10);

        let input_label = Label::new(None);
        input_label.set_halign(Align::Start);
        input_label.set_xalign(0.0);

        let input_entry = Entry::new();

        panel.append(&input_label);
        panel.append(&input_entry);

        Self {
            input_field_box: panel,
            input_label,
            input_entry
        }
    }

    pub fn set_input_callback(&self, callback: impl Fn(&Entry) + 'static) {
        self.input_entry.connect_changed(callback);
    }
}