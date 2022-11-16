use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};

use super::MType;


#[derive(Debug, Atomic)]
pub struct MInt(i32);

impl Object for MInt {
    fn type_id(&self) -> MType {
        MType::MInt
    }
}

impl MInt {
    pub fn new(int: i32) -> Self {
        Self(int)
    }
}
