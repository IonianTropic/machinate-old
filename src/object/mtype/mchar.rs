use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};

use super::MType;


#[derive(Debug, Atomic)]
pub struct MChar(char);

impl Object for MChar {
    fn type_id(&self) -> MType {
        MType::MChar
    }
}

impl MChar {
    pub fn new(ch: char) -> Self {
        Self(ch)
    }
}
