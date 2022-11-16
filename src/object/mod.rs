use std::{cell::{Ref, RefMut, RefCell}, fmt::Debug, rc::Rc};

use self::mtype::MType;

pub mod tests;
pub mod mtype;
mod list;

trait Atomic {
    fn is_atom(&self) -> bool;
    fn wrap(self) -> ObjPtr;
    fn car(&self) -> Option<Ref<dyn Object>>;
    fn cdr(&self) -> Option<Ref<dyn Object>>;
    fn car_mut(&self) -> Option<RefMut<dyn Object>>;
    fn cdr_mut(&self) -> Option<RefMut<dyn Object>>;
}

trait Object: Atomic + Debug {
    fn type_id(&self) -> MType;
}

#[derive(Debug, Clone)]
pub struct ObjPtr(Rc<RefCell<dyn Object>>);

impl ObjPtr {
    fn borrow(&self) -> Ref<dyn Object> {
        self.0.borrow()
    }
    fn borrow_mut(&self) -> RefMut<dyn Object> {
        self.0.borrow_mut()
    }
    fn type_id(&self) -> MType {
        self.borrow().type_id()
    }
    fn t_equals(&self, type_id: MType) -> bool {
        self.type_id() == type_id
    }
}
