use gtk::prelude::{OrientableExt, WidgetExt};
use crate::types::GTKBox;

pub struct BoxBuilder {
    container_box: GTKBox,
}

impl BoxBuilder {
    pub fn new(id_name: &str) -> Self {
        let container_box = GTKBox::new(gtk::Orientation::Vertical, 10);
        container_box.set_widget_name(id_name);

        Self {
            container_box
        }
    }

    pub fn set_margin(&self, margin: u32) -> &Self {
        self.set_margin_top(margin);
        self.set_margin_bottom(margin);
        self.set_margin_start(margin);
        self.set_margin_end(margin);
        self
    }

    pub fn set_margin_top(&self, margin: u32) -> &Self {
        self.container_box.set_margin_top(margin as i32);
        self
    }

    pub fn set_margin_bottom(&self, margin: u32) -> &Self {
        self.container_box.set_margin_bottom(margin as i32);
        self
    }

    pub fn set_margin_start(&self, margin: u32) -> &Self {
        self.container_box.set_margin_start(margin as i32);
        self
    }

    pub fn set_margin_end(&self, margin: u32) -> &Self {
        self.container_box.set_margin_end(margin as i32);
        self
    }

    pub fn set_orientation(&self, orientation: gtk::Orientation) -> &Self {
        self.container_box.set_orientation(orientation);
        self
    }

    pub fn set_width(&self, width: u32) -> &Self {
        self.container_box.set_width_request(width as i32);
        self
    }

    pub fn set_height(&self, height: u32) -> &Self {
        self.container_box.set_height_request(height as i32);
        self
    }

    pub fn set_full_width(&self, state: bool) -> &Self {
        self.container_box.set_hexpand(state);
        self
    }

    pub fn set_full_height(&self, state: bool) -> &Self {
        self.container_box.set_vexpand(state);
        self
    }

    pub fn set_class(&self, class: &str) -> &Self {
        self.container_box.add_css_class(class);
        self
    }

    pub fn set_focusable(&self, state: bool) -> &Self {
        self.container_box.set_focusable(state);
        self
    }

    pub fn set_can_focus(&self, state: bool) -> &Self {
        self.container_box.set_can_focus(state);
        self
    }

    pub fn build(&self) -> GTKBox {
        self.container_box.clone()
    }
}