pub trait LabeledControl {
    fn set_text(&self, text: &str);

    fn set_text_width(&self, width: u32);
}