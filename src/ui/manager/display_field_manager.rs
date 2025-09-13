use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::display_field::DisplayField;
use crate::ui::manager::display_field_manager::DisplayFieldEvent::VisibilityChanged;
use crate::utils::RcMut;

pub struct DisplayFieldManager {
    display_field: RcMut<DisplayField>
}

pub enum DisplayFieldEvent {
    VisibilityChanged(bool),
}

impl DisplayFieldManager {
    pub fn new(display_field: RcMut<DisplayField>) -> Self {
        Self {
            display_field
        }
    }

    pub fn send_event(&self, event: DisplayFieldEvent) {
        match event {
            VisibilityChanged(visible) => {
                if visible {
                    self.display_field.borrow_mut().enable_control();
                } else {
                    self.display_field.borrow_mut().disable_control();
                }
            }
        }
    }
}