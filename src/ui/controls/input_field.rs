use gtk::{Align, Entry, Label, Orientation};
use gtk::prelude::{BoxExt, EditableExt, EntryExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::boxes::DEFAULT_MARGIN;
use crate::ui::controls::Control;
use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::states::input_field_state::InputFieldState;
use crate::ui::updatable_control::UpdatableControl;

pub struct InputField {
    input_field_box: GTKBox,
    input_label: Label,
    input_entry: Entry
}

impl Control for InputField {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.input_field_box
    }
}

impl UpdatableControl<InputFieldState> for InputField {
    fn update_ui(&mut self, state: InputFieldState) {
        self.input_label.set_text(state.label_text.as_str());
        self.input_entry.set_placeholder_text(Some(state.placeholder_text.as_str()));

        if let Some(text) = state.entry_text && !text.is_empty() {
            self.input_entry.set_text(text.as_str());
        } else {
            self.input_entry.set_text("");
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
        let panel = BoxBuilder::new("input-field")
            .set_orientation(Orientation::Vertical)
            .set_margin_bottom(DEFAULT_MARGIN)
            .build();

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