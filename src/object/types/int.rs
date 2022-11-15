use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};

#[derive(Debug, Object)]
pub struct Int(i32);

impl Int {
    pub fn new(int: i32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Int(int)) ) )
    }
}
