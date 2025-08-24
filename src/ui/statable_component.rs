pub trait StatableComponent<State> {
    fn update_state(&mut self, state: State);
}