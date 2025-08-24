use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::controls::startup_program_field::StartupProgramField;

pub struct StartupProgramFieldManager {
    startup_program_field: Rc<RefCell<StartupProgramField>>,
}

pub enum StartupProgramFieldEvent {
    SelectionChanged(Option<String>),
}

impl StartupProgramFieldManager {
    pub fn new(startup_program_field: Rc<RefCell<StartupProgramField>>) -> Self {
        Self {
            startup_program_field
        }
    }

    pub fn send_event(&self, event: StartupProgramFieldEvent) {
        match event {
            StartupProgramFieldEvent::SelectionChanged(program) => {
                let startup_program_field_ref = self.startup_program_field.borrow();
                startup_program_field_ref.change_input_access(program.clone());
                startup_program_field_ref.set_program_fields(program.clone());
            }
        }
    }
}