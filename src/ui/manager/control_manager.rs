use crate::utils::RcMut;

pub trait ControlManager<Control, ControlEvent> {
    fn send_event(&self, event: ControlEvent);

    fn get_control(&self) -> RcMut<Control>;
}