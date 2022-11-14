use std::{cell::RefCell, rc::Rc, pin::Pin};
use machinate::Object;

trait Object {
    fn is_atom(&self) -> bool;
}

#[derive(Object)]
struct Cons {
    car: Pin<Rc<RefCell<dyn Object>>>,
    cdr: Pin<Rc<RefCell<dyn Object>>>,
}

#[derive(Object)]
struct Nil();

