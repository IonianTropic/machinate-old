use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};

#[derive(Debug, Object)]
pub struct Nil;

impl Nil {
    pub fn new() -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Nil) ) )
    }
}
