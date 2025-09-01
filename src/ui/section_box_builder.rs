use gtk::{Label, Orientation, Separator};
use gtk::prelude::{BoxExt, WidgetExt};

pub struct SectionBoxBuilder {
    is_building: bool,
    section_box: gtk::Box,
}

impl SectionBoxBuilder {
    pub fn new() -> Self {
        let section_box = gtk::Box::new(Orientation::Vertical, 10);
        section_box.set_margin_bottom(20);

        Self {
            is_building: true,
            section_box
        }
    }

    pub fn create_header_elements(&mut self, header_name: &str) -> &mut Self {
        let section_header_label = Label::new(Some(header_name));
        let separator = Separator::new(Orientation::Horizontal);
        separator.set_margin_bottom(10);

        self.section_box.append(&section_header_label);
        self.section_box.append(&separator);
        self
    }

    pub fn build(&mut self) -> Result<gtk::Box, String> {
        if !self.is_building {
            return Err("SectionBoxBuilder is already built".to_string());
        }
        
        self.is_building = false;
        Ok(self.section_box.clone())
    }
}