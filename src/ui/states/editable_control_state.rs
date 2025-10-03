#[derive(Clone, Default)]
pub enum EditMode {
    #[default]
    Locked,
    Edit
}

#[derive(Clone, Default)]
pub struct EditableControlState {
    pub edit_mode: EditMode
}