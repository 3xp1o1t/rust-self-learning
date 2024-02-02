fn add_one(value: i32) -> i32 {
    value + 1
}

// function es de tipo puntero a función por que implementa fn
fn do_twice(function: fn(i32) -> i32, value: i32) -> i32 {
    function(value) + function(value)
}

fn main() {
    let add_one_twice = do_twice(add_one, 10);
    println!("{}", add_one_twice);
}
