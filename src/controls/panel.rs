pub mod general_panel;
pub mod display_panel;

pub trait Panel {
    fn get_widget(&self) -> &gtk::Box;
}