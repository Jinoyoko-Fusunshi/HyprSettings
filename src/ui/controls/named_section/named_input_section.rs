use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Entry, Label, Orientation};
use gtk::prelude::{BoxExt, EditableExt, EntryExt, WidgetExt};
use crate::ui::controls::panel::Panel;
use crate::settings::hyprland_settings::HyprlandSettings;

#[derive(Clone)]
pub struct NamedInputSection {
    panel: gtk::Box,
    input_box: Entry
}

impl Panel for NamedInputSection {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.panel
    }
}

impl NamedInputSection {
    pub fn new (
        label_text: &str,
        input_placeholder_text: &str,
        input_text: Option<String>
    ) -> Self
    {
        let panel = gtk::Box::new(Orientation::Vertical, 0);
        panel.set_margin_bottom(10);

        let input_label = Label::new(Some(label_text));
        input_label.set_halign(Align::Start);
        input_label.set_xalign(0.0);

        let input_box = Entry::new();

        if let Some(text) = input_text {
            input_box.set_text(text.as_str());
        } else {
            input_box.set_placeholder_text(Some(input_placeholder_text));
        }

        panel.append(&input_label);
        panel.append(&input_box);

        Self {
            panel,
            input_box
        }
    }

    pub fn set_input_callback(&self, callback: impl Fn(&Entry) + 'static) {
        self.input_box.connect_changed(callback);
    }

    pub fn set_active(&self, active: bool) {
        self.input_box.set_sensitive(active);
    }
}