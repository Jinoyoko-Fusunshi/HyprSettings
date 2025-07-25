use std::cell::RefCell;
use std::rc::Rc;
use crate::settings_container::SettingsContainer;

pub mod general_panel;
pub mod display_panel;
pub mod info_panel;
pub mod appearance_panel;
pub mod startup_programs_panel;
pub mod key_binds_panel;
mod startup_program_entry_row;

pub trait Panel {
    fn reload_settings(&self, settings: &Rc<RefCell<SettingsContainer>>);

    fn get_widget(&self) -> &gtk::Box;
}