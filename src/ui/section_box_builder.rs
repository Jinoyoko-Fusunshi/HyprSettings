use gtk::{Label, Orientation, Separator};
use gtk::prelude::BoxExt;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;

pub struct SectionBoxBuilder {
    is_building: bool,
    section_box: GTKBox,
}

impl SectionBoxBuilder {
    pub fn new(id_name: &str, margin: u32) -> Self {
        let section_box = BoxBuilder::new(id_name)
            .set_margin(margin)
            .build();

        Self {
            is_building: true,
            section_box
        }
    }

    pub fn create_header_elements(&mut self, header_name: &str) -> &mut Self {
        let section_header_label = Label::new(Some(header_name));
        let separator = Separator::new(Orientation::Horizontal);

        self.section_box.append(&section_header_label);
        self.section_box.append(&separator);
        self
    }

    pub fn build(&mut self) -> Result<GTKBox, String> {
        if !self.is_building {
            return Err("SectionBoxBuilder is already built".to_string());
        }
        
        self.is_building = false;
        Ok(self.section_box.clone())
    }
}