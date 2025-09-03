#[derive(Clone)]
pub struct SelectionBoxState {
    pub label_text: String,
    pub selected_option: Option<String>,
    pub options: Vec<String>,
}