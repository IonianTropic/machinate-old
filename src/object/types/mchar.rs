use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};

#[derive(Debug, Object)]
pub struct MChar(char);

impl MChar {
    pub fn new(ch: char) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new( MChar(ch) ) ) )
    }
}
