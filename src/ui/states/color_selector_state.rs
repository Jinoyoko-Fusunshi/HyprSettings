use crate::models::rgba_color::RGBAColor;

#[derive(Clone)]
pub struct ColorSelectorState {
    pub label_text: String,
    pub selected_color: Option<RGBAColor>,
}