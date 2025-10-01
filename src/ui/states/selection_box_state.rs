#[derive(Clone, Default)]
pub struct SelectionBoxState {
    pub label_text: String,
    pub selected_option: Option<String>,
    pub options: Vec<String>,
}

impl SelectionBoxState {
    pub fn get_false_true_options() -> Vec<String> {
        vec!["false".to_string(), "true".to_string()]
    }
}