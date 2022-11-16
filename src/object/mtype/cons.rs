use std::{cell::{Ref, RefMut, RefCell}, rc::Rc};

use crate::object::{Atomic, Object, ObjPtr};


#[derive(Debug)]
pub struct Cons {
    car: ObjPtr,
    cdr: ObjPtr,
}

impl Atomic for Cons {
    fn is_atom(&self) -> bool {
        false
    }
    fn car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.0.borrow())
    }
    fn cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.0.borrow())
    }
    fn car_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.car.0.borrow_mut())
    }
    fn cdr_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.cdr.0.borrow_mut())
    }
}

impl Object for Cons {
    fn type_id(&self) -> u64 {
        2
    }
}

impl Cons {
    pub fn new(car: ObjPtr, cdr: ObjPtr) -> ObjPtr {
        ObjPtr(
            Rc::new(
                RefCell::new(
                    Self {
                        car,
                        cdr,
                    }
                )
            )
        )
    }
}
