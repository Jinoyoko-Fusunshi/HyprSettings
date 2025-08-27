use crate::ui::controls::keybinds::{
    ALT_KEY, CONTROL_KEY, GTK_ALT_KEY, GTK_CONTROL_KEY, GTK_SHIFT_KEY, GTK_SUPER_KEY, SHIFT_KEY,
    SUPER_KEY
};

pub struct KeybindConverter;

impl KeybindConverter {
    pub fn convert_to_real_name(key: String) -> String {
        match key.as_str() {
            GTK_CONTROL_KEY => CONTROL_KEY.to_string(),
            GTK_SHIFT_KEY => SHIFT_KEY.to_string(),
            GTK_ALT_KEY => ALT_KEY.to_string(),
            GTK_SUPER_KEY => SUPER_KEY.to_string(),
            _ => key,
        }
    }
}