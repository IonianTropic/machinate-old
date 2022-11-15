use std::{cell::{RefCell, Ref, RefMut}, rc::Rc, fmt::Debug};

use machinate::Object;

pub fn test_obj() {
    let minus = Symbol::new(String::from("-"));
    let minuend = Int::new(21);
    let subtrahend = Int::new(7);
    let list = Cons::new(minus, Cons::new(minuend, Cons::new(subtrahend, Nil::new())));
    println!(
        "First element: {:?}\nSecond element: {:?}\nThird element: {:?}",
        list.0.borrow().__car().unwrap(),
        list.0.borrow().__cdr().unwrap().__car().unwrap(),
        list.0.borrow().__cdr().unwrap().__cdr().unwrap().__car().unwrap(),
    );
}

trait Object: Debug {
    fn __atom(&self) -> bool;
    fn __car(&self) -> Option<Ref<dyn Object>>;
    fn __cdr(&self) -> Option<Ref<dyn Object>>;
    fn __car_mut(&self) -> Option<RefMut<dyn Object>>;
    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>>;
}

#[derive(Debug)]
struct ObjPtr(Rc<RefCell<dyn Object>>);

#[derive(Debug)]
struct Cons {
    car: ObjPtr,
    cdr: ObjPtr,
}

impl Cons {
    fn new(car: ObjPtr, cdr: ObjPtr) -> ObjPtr {
        ObjPtr(
            Rc::new(
                RefCell::new(
                    Self {
                        car,
                        cdr,
                    }
                )))
    }
}

impl Object for Cons {
    fn __atom(&self) -> bool {
        false
    }
    
    fn __car(&self) -> Option<Ref<dyn Object>> {
        Some(self.car.0.borrow())
    }

    fn __cdr(&self) -> Option<Ref<dyn Object>> {
        Some(self.cdr.0.borrow())
    }

    fn __car_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.car.0.borrow_mut())
    }

    fn __cdr_mut(&self) -> Option<RefMut<dyn Object>> {
        Some(self.cdr.0.borrow_mut())
    }
}

#[derive(Debug, Object)]
struct Nil;

impl Nil {
    fn new() -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Nil) ) )
    }
}

#[derive(Debug, Object)]
struct Int(i32);

impl Int {
    fn new(int: i32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Int(int)) ) )
    }
}

#[derive(Debug, Object)]
struct Float(f32);

impl Float {
    fn new(float: f32) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Float(float)) ) )
    }
}

#[derive(Debug, Object)]
struct Char(char);

impl Char {
    fn new(ch: char) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Char(ch)) ) )
    }
}

#[derive(Debug, Object)]
struct MString(String);

impl MString {
    fn new(mstr: String) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(MString(mstr)) ) )
    }
}

#[derive(Debug, Object)]
struct Symbol(String);

impl Symbol {
    fn new(mstr: String) -> ObjPtr {
        ObjPtr( Rc::new( RefCell::new(Symbol(mstr)) ) )
    }
}
