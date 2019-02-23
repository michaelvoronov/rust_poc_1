use std::{cell::RefCell, rc::Rc};
use arraydeque::{ArrayDeque, Wrapping};

struct SomeStruct {
    field: i32
}

struct BigStruct {
    field: ArrayDeque<[Rc<RefCell<SomeStruct>>; 1024], Wrapping>
}

impl BigStruct {
    pub fn new() -> Self {
        BigStruct {
            field: ArrayDeque::new()
        }
    }

    pub fn foo(&self) -> i32 {
        1
    }
}

thread_local! {
    static GM: RefCell<BigStruct> = RefCell::new(BigStruct::new())
}

fn main() {
    GM.with(|gm|
        gm.borrow().foo()
    );

    println!("finish...");
}
