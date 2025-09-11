use gtk::{Align, ColorButton, Label, Orientation};
use gtk::prelude::{BoxExt, ColorChooserExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::labeled_control::LabeledControl;
use crate::ui::states::color_selector_state::ColorSelectorState;
use crate::ui::updatable_control::UpdatableControl;

pub struct ColorSelector {
    color_button_box: GTKBox,
    color_button_label: Label,
    color_button: ColorButton,
}

impl Control for ColorSelector {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.color_button_box
    }
}

impl LabeledControl for ColorSelector {
    fn set_text_width(&self, width: u32) {
        self.color_button_label.set_width_request(width as i32);
    }
}

impl UpdatableControl<ColorSelectorState> for ColorSelector {
    fn update_ui(&mut self, state: ColorSelectorState) {
        self.color_button_label.set_text(&state.label_text);
        if let Some(color) = state.selected_color {
            self.color_button.set_rgba(color.get_rgba());
        }
    }
}

impl ColorSelector {
    pub fn new() -> ColorSelector {
        let color_button_box = BoxBuilder::new("color-selector")
            .set_orientation(Orientation::Horizontal)
            .build();

        let color_button_label = Label::new(None);
        color_button_label.set_halign(Align::Start);
        color_button_label.set_xalign(0.0);
        
        let color_button = ColorButton::new();
        color_button.set_use_alpha(true);
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