use std::{rc::Rc, cell::RefCell};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};

use super::MType;


#[derive(Debug, Atomic)]
pub struct Nil;

impl Object for Nil {
    fn type_id(&self) -> MType {
        MType::Nil
    }
}

impl Nil {
    pub fn new() -> Self {
        Self
    }
}
