use super::{mtype::{cons::Cons, nil::Nil, MType}, ObjPtr, Atomic};

struct List {
    head: ObjPtr,
    tail: ObjPtr,
    len: usize,
}

impl List {

    fn _push_front_cons(&mut self, mut cons: Cons) {

        cons.cdr = self.head.clone();

        if self.head.equals(MType::Nil) {
            
        } else {

        }

        self.head = cons.wrap();
    }

}

impl List {
    pub fn new() -> Self {
        List { head: Nil::new().wrap(), tail: Nil::new().wrap(), len: 0 }
    }
}

struct IntoIter {
    list: List
}

impl Iterator for IntoIter {
    type Item = ObjPtr;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl IntoIterator for List {
    type Item = ObjPtr;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}
