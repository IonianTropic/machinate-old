use std::{cell::{RefCell, Ref}, rc::Rc};

trait Object {
    fn is_atom(&self) -> bool;
    fn get_car(&self) -> Option<Ref<dyn Object>>;
    fn get_cdr(&self) -> Option<Ref<dyn Object>>;
}

struct Cons {
    car: Rc<RefCell<dyn Object>>,
    cdr: Rc<RefCell<dyn Object>>,
}

impl Object for Cons {
    fn is_atom(&self) -> bool {
        false
    }

    fn get_car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.borrow())
    }

    fn get_cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.borrow())
    }
}

