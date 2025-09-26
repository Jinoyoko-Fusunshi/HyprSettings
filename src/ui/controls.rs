use crate::types::GTKBox;

pub mod activable_control;
pub mod editable_control_element;
pub mod color_selector;
pub mod input_field;
pub mod selection_box;
pub mod spin_button;
pub mod monitor_field;
pub mod keybinds;
pub mod startup_program_field;
pub mod navigation;
pub mod settings_switcher;
pub mod monitor;
pub mod monitor_configurator;

pub trait Control {
    fn init_events(&self);

    fn get_widget(&self) -> &GTKBox;
}