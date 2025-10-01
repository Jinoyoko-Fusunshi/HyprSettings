#[derive(Clone, Default)]
pub struct InputFieldState {
    pub label_text: String,
    pub entry_text: Option<String>,
    pub placeholder_text: String,
}