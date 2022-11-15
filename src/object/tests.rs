use crate::object::types::{symbol::Symbol, int::Int, cons::Cons, nil::Nil};

// TODO: write full for object system
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
