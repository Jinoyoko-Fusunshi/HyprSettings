use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Align, Button};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::ui::controls::editable_control::EditableControl;
use crate::ui::controls::panel::Panel;

#[derive(Clone)]
pub enum EditMode {
    Locked,
    Edit
}

#[derive(Clone)]
struct EditableControlModel {
    edit_mode: EditMode,
}

#[derive(Clone)]
pub struct EditableControlElement<Element: EditableControl + Panel + Clone + 'static> {
    editable_control_element_box: gtk::Box,
    editable_control: Element,
    mode_toggle_button: Button,
    model: Rc<RefCell<EditableControlModel>>
}

impl<Element: EditableControl + Panel + Clone> Panel for EditableControlElement<Element> {
    fn reload_settings(&self, _: &Rc<RefCell<HyprlandSettings>>) {}

    fn get_container_box(&self) -> &gtk::Box {
        &self.editable_control_element_box
    }
}

impl<Element: EditableControl + Panel + Clone> EditableControlElement<Element> {
    pub fn new(editable_control: Element, edit_mode: EditMode) -> Self {
        let editable_control_element_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        let edit_mode_text = match edit_mode {
            EditMode::Locked => "✏️",
            EditMode::Edit => "✅"
        };

        match edit_mode {
            EditMode::Locked => editable_control.disable_control(),
            EditMode::Edit => editable_control.enable_control()
        }

        let mode_toggle_button = Button::with_label(edit_mode_text);
        mode_toggle_button.set_vexpand(false);
        mode_toggle_button.set_valign(Align::Center);

        editable_control_element_box.append(editable_control.get_container_box());
        editable_control_element_box.append(&mode_toggle_button);

        let model = EditableControlModel {
            edit_mode
        };

        Self {
            editable_control_element_box,
            editable_control,
            mode_toggle_button,
            model: Rc::new(RefCell::new(model))
        }
    }

    pub fn set_toggle_button_click_callback(
        &self, settings: Rc<RefCell<HyprlandSettings>>,
        save_callback: impl Fn(Rc<RefCell<HyprlandSettings>>) + 'static
    ) {
        let this = self.clone();
        let mode_toggle_button_callback = move |_: &Button| {
            let mut model = this.model.borrow_mut();
            match model.edit_mode {
                EditMode::Locked => {
                    this.editable_control.enable_control();
                    this.mode_toggle_button.set_label("✅");
                    model.edit_mode = EditMode::Edit;
                },
                EditMode::Edit => {
                    this.editable_control.disable_control();
                    this.mode_toggle_button.set_label("✏️");
                    model.edit_mode = EditMode::Locked;
                    save_callback(settings.clone());
                }
            }
        };

        self.mode_toggle_button.connect_clicked(mode_toggle_button_callback);
    }
}