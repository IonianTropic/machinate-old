use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};

use super::MType;


#[derive(Debug, Atomic)]
pub struct Symbol(String);

impl Object for Symbol {
    fn type_id(&self) -> MType {
        MType::Symbol
    }
}

impl Symbol {
    pub fn new(s: &str) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( Symbol(String::from(s)) ) ) )
    }
}
