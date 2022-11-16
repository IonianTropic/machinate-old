use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};


#[derive(Debug, Atomic)]
pub struct MInt(i32);

impl Object for MInt {
    fn type_id(&self) -> u64 {
        4
    }
}

impl MInt {
    pub fn new(int: i32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( MInt(int) ) ) )
    }
}
