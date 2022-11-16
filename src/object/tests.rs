use crate::object::mtype::{symbol::Symbol, mint::MInt, cons::Cons, nil::Nil, mchar::MChar};

// TODO: write full for object system
pub fn test_obj() {
    let _h = MChar::new('h');
    let minus = Symbol::new("-");
    let minuend = MInt::new(21);
    let subtrahend = MInt::new(7);
    let list = Cons::new(minus, Cons::new(minuend, Cons::new(subtrahend, Nil::new())));
    println!(
        "First element: {:?}\nSecond element: {:?}\nThird element: {:?}",
        list.0.borrow().car().unwrap(),
        list.0.borrow().cdr().unwrap().car().unwrap(),
        list.0.borrow().cdr().unwrap().cdr().unwrap().car().unwrap(),
    );
}
