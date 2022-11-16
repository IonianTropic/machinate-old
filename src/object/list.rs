use super::{mtype::{cons::Cons, nil::Nil, MType}, Atomic, ObjPtr};


/// singly linked list implemented with cons cells
/// clos stores the head and tail of the list in a cons cell
/// where head -> `self.clos.car` and tail -> `self.clos.cdr`
#[derive(Debug)]
struct List {
    clos: Cons,
    len: usize,
}

// private
impl List {
    fn push_front_cell(&mut self, mut cons: Cons) {
        /* 
         * HEAD -> TAIL
         * cons -> HEAD -> TAIL
         * HEAD -> ... -> TAIL
         * 
         */
        cons.cdr = self.clos.car.clone();
        let wrap = cons.wrap();
        match self.clos.car.type_id() {
            MType::Nil => self.clos.cdr = wrap.clone(),
            MType::Cons => (),
            _ => panic!("Encounted non null atom in list cells")
        }
        self.clos.car = wrap;
        self.len += 1;
    }

    fn pop_front_cell(&mut self) -> ObjPtr {
        /*
         * HEAD -> HEAD.next -> ... -> TAIL
         * HEAD -x HEAD.next -> ... -> TAIL
         */
        // get mut? clos.car
        // get return value clos.car.car
        // null clos.car.cdr
        let mut head = self.clos.car.borrow_mut();
        let ret = head.car().unwrap();
        self.clos.car = head.cdr().take().unwrap();
        self.len -= 1;
    }

    fn push_back_cell(&mut self, mut cons: Cons) {
        /*
         * HEAD -> TAIL
         * HEAD -> TAIL -> cons
         * HEAD -> ... -> TAIL
         */
        cons.cdr = Nil::new().wrap();
        let wrap = cons.wrap();
        match self.clos.cdr.type_id() {
            MType::Nil => self.clos.car = wrap.clone(),
            MType::Cons => {
                let wrcl = wrap.clone();
                let mut _tail_next = self.clos.car.borrow_mut();
                _tail_next = wrcl.borrow_mut();
            }
            _ => panic!("Encounted non null atom in list cells")
        }
        self.clos.car = wrap;
        self.len += 1;
    }
}

impl List {
    pub fn new() -> Self {
        Self { 
            clos: Cons::new(
                Nil::new().wrap(),
                Nil::new().wrap()),
            len: 0,
        }
    }
}
