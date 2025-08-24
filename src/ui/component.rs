pub trait Component {
    fn init_events(&self);

    fn get_widget(&self) -> &gtk::Box;
}