use gtk::{Align, GestureClick, Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::css_styles::CSSStyles;
use crate::ui::states::monitor_state::DisplayElementState;
use crate::ui::updatable_control::UpdatableControl;

#[derive(Clone)]
pub struct Monitor {
    monitor_box: GTKBox,
    port_label: Label,
    offset_label: Label,
}

impl Control for Monitor {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.monitor_box
    }
}

impl UpdatableControl<DisplayElementState> for Monitor {
    fn update_ui(&mut self, state: DisplayElementState) {
        let size = state.size;

        self.monitor_box.set_width_request(size.get_x() as i32);
        self.monitor_box.set_height_request(size.get_y() as i32);
        self.port_label.set_text(&state.port_name);

        let display_text = format!(
            "x: {:.0} y: {:.0}", state.position.get_x() * 10.0, state.position.get_y() * 10.0
        );
        self.offset_label.set_text(&display_text);
    }
}

impl Monitor {
    pub fn new() -> Self {
        let display_element_box = BoxBuilder::new("display-element")
            .set_class(CSSStyles::MONITOR_BOX)
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
            monitor_box: display_element_box,
            port_label: display_port_label,
            offset_label: display_offset_label,
        }
    }

    pub fn focus(&self) {
        self.monitor_box.grab_focus();
    }

    pub fn set_click_controller(&self, click_controller: GestureClick) {
        self.monitor_box.add_controller(click_controller);
    }
}