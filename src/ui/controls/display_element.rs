use gtk::{Align, GestureClick, Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::css_styles::CSSStyles;
use crate::ui::states::display_element_state::DisplayElementState;
use crate::ui::updatable_control::UpdatableControl;

#[derive(Clone)]
pub struct DisplayElement {
    display_element_box: GTKBox,
    display_port_label: Label,
    display_offset_label: Label,
}

impl Control for DisplayElement {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.display_element_box
    }
}

impl UpdatableControl<DisplayElementState> for DisplayElement {
    fn update_ui(&mut self, state: DisplayElementState) {
        let size = state.size;

        self.display_element_box.set_width_request(size.get_x() as i32);
        self.display_element_box.set_height_request(size.get_y() as i32);
        self.display_port_label.set_text(&state.port_name);

        let display_text = format!(
            "x: {:.0}, y: {:.0}", state.position.get_x() * 10.0, state.position.get_y() * 10.0
        );
        self.display_offset_label.set_text(&display_text);
    }
}

impl DisplayElement {
    pub fn new() -> Self {
        let display_element_box = BoxBuilder::new("display-element")
            .set_class(CSSStyles::DISPLAY_ELEMENT_BOX)
            .build();
        display_element_box.set_can_focus(true);
        display_element_box.set_focusable(true);
        display_element_box.set_vexpand(false);
        display_element_box.set_hexpand(false);

        let display_information_box = BoxBuilder::new("display-information")
            .set_orientation(Orientation::Vertical)
            .build();
        display_information_box.set_valign(Align::Center);
        display_information_box.set_halign(Align::Center);
        display_information_box.set_vexpand(true);
        display_information_box.set_hexpand(true);

        let display_port_label = Label::new(None);
        let display_offset_label = Label::new(None);
        display_information_box.append(&display_port_label);
        display_information_box.append(&display_offset_label);

        display_element_box.append(&display_information_box);

        Self {
            display_element_box,
            display_port_label,
            display_offset_label,
        }
    }

    pub fn focus(&self) {
        self.display_element_box.grab_focus();
    }

    pub fn set_click_controller(&self, click_controller: GestureClick) {
        self.display_element_box.add_controller(click_controller);
    }
}