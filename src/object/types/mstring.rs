use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};

#[derive(Debug, Object)]
pub struct MString(String);

impl MString {
    pub fn new(mstr: String) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(MString(mstr)) ) )
    }
}
