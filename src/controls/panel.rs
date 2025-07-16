pub mod general_panel;

pub trait Panel {
    fn get_widget(&self) -> &gtk::Box;
}