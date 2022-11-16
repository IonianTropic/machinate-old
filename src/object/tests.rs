use crate::object::{mtype::{cons::Cons, mchar::MChar}};

use super::Atomic;

// TODO: write full for object system
pub fn test_obj() {
    // let minus = Symbol::new("-");
    // let minuend = MInt::new(21);
    // let subtrahend = MInt::new(7);

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
    let mut _h = MChar::new('h');
    let e = MChar::new('e');
    _h = e;
    let mut _l = MChar::new('l').wrap();
    let o = MChar::new('o').wrap();
    _l = o;
    let singleton = Cons::new(_l.clone(), _l);
    println!("o: {:?}\nalso o: {:?}", singleton.car().unwrap(), singleton.cdr().unwrap());
    
    // mcar = mwrap.clone();
    // let raw = wrap.0.as_ptr();

    // let list_ref = list.borrow()
    // println!(
    //     "First element: {:?}\nSecond element: {:?}\nThird element: {:?}",
    //     list_ref.car().unwrap(),
    //     list_ref.cdr().unwrap().car().unwrap(),
    //     list_ref.cdr().unwrap().cdr().unwrap().car().unwrap(),
    // );
}
