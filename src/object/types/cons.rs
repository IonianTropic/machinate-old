use std::{rc::Rc, cell::{RefCell, RefMut, Ref}};

use crate::object::{ObjPtr, Object};


#[derive(Debug)]
pub struct Cons {
    car: ObjPtr,
    cdr: ObjPtr,
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
                )))
    }
}

impl Object for Cons {
    fn __atom(&self) -> bool {
        false
    }
    
    fn __car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.0.borrow())
    }

    fn __cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.0.borrow())
    }

    fn __car_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.car.0.borrow_mut())
    }

    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.cdr.0.borrow_mut())
    }
}
