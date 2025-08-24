pub trait UpdatableComponent<State> {
    fn update_ui(&mut self, state: State);
}