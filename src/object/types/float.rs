use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use machinate::Object;

use crate::object::{Object, ObjPtr};


#[derive(Debug, Object)]
pub struct Float(f32);

impl Float {
    pub fn new(float: f32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Float(float)) ) )
    }
}
