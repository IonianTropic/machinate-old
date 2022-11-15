use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};

#[derive(Debug, Object)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(mstr: String) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Symbol(mstr)) ) )
    }
}
