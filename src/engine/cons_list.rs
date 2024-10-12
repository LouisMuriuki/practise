#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use super::cons_list::List::{Cons, Nil};

pub fn create_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
