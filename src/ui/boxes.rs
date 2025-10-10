use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::css_styles::CSSStyles;

pub struct Boxes;

pub const DEFAULT_MARGIN: u32 = 10;
pub const DEFAULT_SPACING: u32 = 10;

impl Boxes {
    pub fn clear_box_content(box_container: &GTKBox) {
        let mut current_widget = box_container.first_child();
        while let Some(child_widget) = current_widget.clone() {
            box_container.remove(&child_widget);
            current_widget = box_container.next_sibling();
        }
    }
    
    pub fn set_margin(box_container: &GTKBox, margin: u32) {
        box_container.set_margin_top(margin as i32);
        box_container.set_margin_bottom(margin as i32);
        box_container.set_margin_start(margin as i32);
        box_container.set_margin_end(margin as i32);
    }
    
    pub fn create_warning_box(warning_text: &str) -> GTKBox {
        let display_error_box = BoxBuilder::new("warning")
            .set_orientation(Orientation::Vertical)
            .build();

        let display_error_label = Label::new(Some(warning_text));
        display_error_label.add_css_class(CSSStyles::WARNING_LABEL);

        display_error_box.append(&display_error_label);
        display_error_box
    }
}