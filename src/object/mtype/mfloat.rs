use std::{cell::RefCell, rc::Rc};

use machinate::Atomic;

use crate::object::{Object, ObjPtr};

use super::MType;


#[derive(Debug, Atomic)]
pub struct MFloat(f32);

impl Object for MFloat {
    fn type_id(&self) -> MType {
        MType::MFloat
    }
}

impl MFloat {
    pub fn new(fp: f32) -> Self {
        Self(fp)
    }
}
