pub trait UpdatableControl<State> {
    fn update_ui(&mut self, state: State);
}