pub trait UpdatableControl<State> {
    fn update_state(&mut self, state: State);

    fn get_current_state(&self) -> State;
}