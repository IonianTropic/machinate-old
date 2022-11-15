use std::{cell::RefCell, rc::Rc};

trait Object {
    fn is_atom(&self) -> bool;
    fn get_car(&self) -> Option<Rc<RefCell<dyn Object>>>;
    fn get_cdr(&self) -> Option<Rc<RefCell<dyn Object>>>;
}

struct Cons {
    car: Rc<RefCell<dyn Object>>,
    cdr: Rc<RefCell<dyn Object>>,
}

// impl Object for Cons {
//     fn is_atom(&self) -> bool {
//         false
//     }

//     fn get_car(&self) -> Option<Rc<RefCell<dyn Object>>> {
//         Some(self.car)
//     }

//     fn get_cdr(&self) -> Option<Rc<RefCell<dyn Object>>> {
//         Some(self.cdr)
//     }
// }

