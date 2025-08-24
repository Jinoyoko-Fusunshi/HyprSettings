pub trait ActivableControl {
    fn enable_control(&self);
    fn disable_control(&self);
}