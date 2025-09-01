pub trait StatableControl<State> {
    fn update_state(&mut self, state: State);
}