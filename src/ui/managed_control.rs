pub trait ManagedControl<Manager> {
    fn init_events_by_manager(&self, manager: Manager);
}