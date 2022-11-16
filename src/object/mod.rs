use std::{cell::{Ref, RefMut, RefCell}, fmt::Debug, rc::Rc};

pub mod tests;
pub mod mtype;

trait Atomic {
    fn is_atom(&self) -> bool;
    fn car(&self) -> Option<Ref<dyn Object>>;
    fn cdr(&self) -> Option<Ref<dyn Object>>;
    fn car_mut(&self) -> Option<RefMut<dyn Object>>;
    fn cdr_mut(&self) -> Option<RefMut<dyn Object>>;
}

trait Object: Debug + Atomic {
    fn type_id(&self) -> u64;
}

#[derive(Debug)]
pub struct ObjPtr(Rc<RefCell<dyn Object>>);
