fn add_one(value: i32) -> i32 {
    value + 1
}

// function es de tipo puntero a función por que implementa fn
fn do_twice(function: fn(i32) -> i32, value: i32) -> i32 {
    function(value) + function(value)
}

// retornar un closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let add_one_twice = do_twice(add_one, 10);
    println!("{}", add_one_twice);

    // return closure
    let closure = returns_closure();
    println!("{}", closure(10));
}
