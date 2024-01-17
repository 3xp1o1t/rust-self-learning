enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    println!("Ejemplo usando Rc<T> smart pointer para conteo de refs!");

    // Lista a = 5 -> 10 -> Nil
    // Lista _b = 3 -> 5 -> 10 -> Nil
    // Lista _c = 4 -> 5 -> 10 -> Nil

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Referencias despues de crear a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Referencias despues de crear _b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Referencias despues de crear _c = {}", Rc::strong_count(&a));
    }

    println!(
        "Referencias despues de que _c salio del scope = {}",
        Rc::strong_count(&a)
    );
}
