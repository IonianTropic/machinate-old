use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

trait Object {
    fn __atom(&self) -> bool;
    fn __car(&self) -> Option<Ref<dyn Object>>;
    fn __cdr(&self) -> Option<Ref<dyn Object>>;
    fn __car_mut(&self) -> Option<RefMut<dyn Object>>;
    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>>;
}

struct Cons {
    car: Rc<RefCell<dyn Object>>,
    cdr: Rc<RefCell<dyn Object>>,
}

impl Object for Cons {
    fn __atom(&self) -> bool {
        false
    }
    
    fn __car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.borrow())
    }

    fn __cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.borrow())
    }

    fn __car_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.car.borrow_mut())
    }

    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.cdr.borrow_mut())
    }
}

#[derive(Object)]
struct Nil;

