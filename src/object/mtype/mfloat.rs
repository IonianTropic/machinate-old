use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};


#[derive(Debug, Atomic)]
pub struct MFloat(f32);

impl Object for MFloat {
    fn type_id(&self) -> u64 {
        4
    }
}

impl MInt {
    pub fn new(int: i32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( MFloat(int) ) ) )
    }
}
