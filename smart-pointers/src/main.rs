use std::mem::drop;
use std::ops::{Deref, Drop};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let gretting = MyBox::new(String::from("Rust"));
    hello(&gretting);

    println!("Trait drop examples");

    let _c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("Other Stuff"),
    };

    println!("CustomSmartPointer created.");

    println!("CustomSmartPointer dropped using mem drop.");

    let _m = CustomSmartPointer {
        data: String::from("Mem drop"),
    };

    println!("CustomSmartPointer MEM DROP version created.");

    drop(_m);

    println!("CustomSmartPointer dropped before the end of main.");
}

// coercion deref
fn hello(name: &str) {
    println!("Hello, {name}");
}
