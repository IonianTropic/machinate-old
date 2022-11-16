use std::{rc::Rc, cell::RefCell};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};


#[derive(Debug, Atomic)]
pub struct Nil;

impl Object for Nil {
    fn type_id(&self) -> u64 {
        0
    }
}

impl Nil {
    pub fn new() -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Nil) ) )
    }
}
