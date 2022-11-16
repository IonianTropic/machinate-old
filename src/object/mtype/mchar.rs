use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};


#[derive(Debug, Atomic)]
pub struct MChar(char);

impl Object for MChar {
    fn type_id(&self) -> u64 {
        4
    }
}

impl MChar {
    pub fn new(ch: char) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( MChar(ch) ) ) )
    }
}
