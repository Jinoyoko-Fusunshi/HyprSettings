use gtk::{Label, Orientation};
use gtk::prelude::{BoxExt, WidgetExt};
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::css_styles::CSSStyles;

pub struct KeySymbol {
    key_symbol_box: GTKBox
}

impl Control for KeySymbol {
    fn get_widget(&self) -> &GTKBox {
        &self.key_symbol_box
    }
}

impl KeySymbol {
    pub fn new(key_name: String) -> Self {
        let key_symbol_box = BoxBuilder::new("key-symbol")
            .set_orientation(Orientation::Vertical)
            .set_full_height(false)
            .set_class(CSSStyles::KEY_SYMBOL_BOX)
            .build();

        let key_label = Label::new(Some(&key_name));
        key_label.add_css_class(CSSStyles::KEY_SYMBOL_BOX);
        key_symbol_box.append(&key_label);

        Self {
            key_symbol_box
        }
    }
}