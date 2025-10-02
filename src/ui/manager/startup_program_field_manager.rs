use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::controls::startup_program_field::StartupProgramField;
use crate::ui::manager::control_manager::ControlManager;
use crate::utils::RcMut;

pub struct StartupProgramFieldManager {
    startup_program_field: Rc<RefCell<StartupProgramField>>,
}

impl ControlManager<StartupProgramField, StartupProgramFieldEvent> for StartupProgramFieldManager {
    fn send_event(&self, event: StartupProgramFieldEvent) {
        match event {
            StartupProgramFieldEvent::SelectionChanged(program) => {
                let startup_program_field_ref = self.startup_program_field.borrow();
                startup_program_field_ref.change_input_access(program.clone());
                startup_program_field_ref.set_program_fields(program.clone());
            }
        }
    }

    fn get_control(&self) -> RcMut<StartupProgramField> {
        self.startup_program_field.clone()
    }
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
}