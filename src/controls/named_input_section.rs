use gtk::{Align, Entry, Label, Orientation};
use gtk::prelude::{BoxExt, EditableExt, EntryExt, WidgetExt};
use crate::controls::panel::Panel;

pub struct NamedInputSection {
    panel: gtk::Box
}

impl Panel for NamedInputSection {
    fn get_widget(&self) -> &gtk::Box {
        &self.panel
    }
}

impl NamedInputSection {
    pub fn new (
        label_text: &str, input_placeholder_text: &str,
        input_change_callback: Option<impl Fn(&Entry) + 'static>
    ) -> Self
    {
        let panel = gtk::Box::new(Orientation::Vertical, 0);
        panel.set_margin_bottom(10);

        let input_label = Label::new(Some(label_text));
        input_label.set_halign(Align::Start);
        input_label.set_xalign(0.0);

        let input_box = Entry::new();
        input_box.set_placeholder_text(Some(input_placeholder_text));

        if let Some(callback) = input_change_callback {
            input_box.connect_changed(callback);
        }

        panel.append(&input_label);
        panel.append(&input_box);

        Self {
            panel
        }
    }
}