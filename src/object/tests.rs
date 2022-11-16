use crate::object::mtype::{symbol::Symbol, mint::MInt, cons::Cons, nil::Nil, mchar::MChar};

// TODO: write full for object system
pub fn test_obj() {
    let minus = Symbol::new("-");
    let minuend = MInt::new(21);
    let subtrahend = MInt::new(7);

    // let list = Cons::new(
    //     minus,
    //     Cons::new(
    //         minuend,
    //         Cons::new(
    //             subtrahend,
    //             Nil::new()
    //         )
    //     )
    // );
    let mut e = MChar::new('e');
    let mut h = MChar::new('h');
    h = e;
    // let list_ref = list.borrow();
    // println!(
    //     "First element: {:?}\nSecond element: {:?}\nThird element: {:?}",
    //     list_ref.car().unwrap(),
    //     list_ref.cdr().unwrap().car().unwrap(),
    //     list_ref.cdr().unwrap().cdr().unwrap().car().unwrap(),
    // );
}
