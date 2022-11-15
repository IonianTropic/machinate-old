use std::{cell::{Ref, RefMut, RefCell}, fmt::Debug, rc::Rc};

pub mod tests;
mod types;

pub trait Object: Debug {
    fn __atom(&self) -> bool;
    fn __car(&self) -> Option<Ref<dyn Object>>;
    fn __cdr(&self) -> Option<Ref<dyn Object>>;
    fn __car_mut(&self) -> Option<RefMut<dyn Object>>;
    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>>;
}

#[derive(Debug)]
pub struct ObjPtr(Rc<RefCell<dyn Object>>);
