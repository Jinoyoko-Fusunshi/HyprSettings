use gtk::{Align, ColorButton, Label};
use gtk::prelude::{BoxExt, ColorChooserExt, WidgetExt};
use crate::settings::rgba_color::RGBAColor;
use crate::ui::component::Component;
use crate::ui::labeled_component::LabeledComponent;
use crate::ui::updatable_component::UpdatableComponent;

pub struct ColorSelectorState {
    pub label_text: String,
    pub selected_color: Option<RGBAColor>,
}

pub struct ColorSelector {
    color_button_box: gtk::Box,
    color_button_label: Label,
    color_button: ColorButton,
}

impl Component for ColorSelector {
    fn init_events(&self) {}

    fn get_widget(&self) -> &gtk::Box {
        &self.color_button_box
    }
}

impl LabeledComponent for ColorSelector {
    fn set_text_width(&self, width: u32) {
        self.color_button_label.set_width_request(width as i32);
    }
}

impl UpdatableComponent<ColorSelectorState> for ColorSelector {
    fn update_ui(&mut self, state: ColorSelectorState) {
        self.color_button_label.set_text(&state.label_text);
        if let Some(color) = state.selected_color {
            self.color_button.set_rgba(color.get_rgba());
        }
    }
}

impl ColorSelector {
    pub fn new() -> ColorSelector {
        let color_button_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        
        let color_button_label = Label::new(None);
        color_button_label.set_halign(Align::Start);
        color_button_label.set_xalign(0.0);
        
        let color_button = ColorButton::new();
        
        color_button_box.append(&color_button_label);
        color_button_box.append(&color_button);
        
        Self {
            color_button_box,
            color_button_label,
            color_button
        }
    }

    pub fn set_color_change(&self, color_change: impl Fn(&ColorButton) + 'static) {
        self.color_button.connect_color_set(color_change);
    }
}