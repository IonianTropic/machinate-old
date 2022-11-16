use std::{cell::{Ref, RefMut, RefCell}, rc::Rc};

use crate::object::{Atomic, Object, ObjPtr};

use super::MType;

/* RECIPES FOR IO
 *
 * Cons cell with both ptrs pointing to the same data
 * 
 * let data = Nil::new().wrap();
 * let singleton = Cons::new(data.clone(), data);
 * 
 * !! Optionally drop data to prevent dynamic dispatch creating more strong references
 * 
 * TODO: Cons cell with ptr(s) pointing to itself
 */

#[derive(Debug, Clone)]
pub struct Cons {
    pub car: ObjPtr,
    pub cdr: ObjPtr,
}

impl Atomic for Cons {
    fn is_atom(&self) -> bool {
        false
    }
    fn car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.borrow())
    }
    fn cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.borrow())
    }
    fn car_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.car.borrow_mut())
    }
    fn cdr_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.cdr.borrow_mut())
    }
    fn wrap(self) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( self ) ) )
    }
}

impl Object for Cons {
    fn type_id(&self) -> MType {
        MType::Cons
    }
}

impl Cons {
    pub fn new(car: ObjPtr, cdr: ObjPtr) -> Self {
        Self { car, cdr, }
    }
}
