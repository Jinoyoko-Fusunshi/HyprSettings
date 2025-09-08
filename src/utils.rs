use std::cell::RefCell;
use std::rc::Rc;

pub type RcMut<T> = Rc<RefCell<T>>;

pub fn new_rc_mut<T>(value: T) -> RcMut<T> {
    Rc::new(RefCell::new(value))
}