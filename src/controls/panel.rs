pub mod general_panel;
pub mod display_panel;
pub mod info_panel;
pub mod appearance_panel;

pub trait Panel {
    fn get_widget(&self) -> &gtk::Box;
}