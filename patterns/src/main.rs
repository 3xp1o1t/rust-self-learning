struct Point {
    x: i32,
    y: i32,
}
fn main() {
    // fold functions keyboard = ctrl + shift + [] - [close ]open
    if_let_pattern();
    while_let_pattern();
    for_pattern();
    params_pattern(&(3, 5));
    match_literals();
    match_named_vars();
    or_pattern();
    char_range_pattern();
    match_struct();
    ignore_values_tuple();
    match_guard_with_or();
    at_binding();
}

fn if_let_pattern() {
    println!("# If let pattern!");
    // if let using else if let and else
    let favorite_color: Option<&str> = None;
    let is_saturday: bool = false;
    // This is the way -> 42.parse = true, others = false.
    let age: Result<u8, _> = "42".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color} as the background!");
    } else if is_saturday {
        println!("Saturday best day");
    } else if let Ok(age) = age {
        // Ok(age) != age -> Shadowing age variable
        if age > 28 {
            println!("Using black as background color!");
        } else {
            println!("Using red as background color!");
        }
    } else {
        println!("Using cyan as background color!");
    }
}

fn while_let_pattern() {
    println!("# While let pattern");
    // poping a vector using while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(8);
    stack.push(222);
    stack.push(58);

    // pop() -> returns an Option<T> - Some = item | None if stack is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_pattern() {
    println!("# For pattern");
    // the value after the reserved keyword `for` it is a pattern
    let values = vec!['r', 'u', 's', 't'];

    for (index, value) in values.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn params_pattern(&(x, y): &(i32, i32)) {
    println!("# Function Params Pattern");
    // Function params can be patterns
    println!("Current location: ({}, {}) ", x, y);
}

fn match_literals() {
    println!("# Match pattern");

    let x = 3;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Anything else"),
    }
}

fn match_named_vars() {
    println!("# Match named variables");
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(y) => println!("Matched, y = {y}"), // This run bc `y` is a new variable in the match context {}
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn or_pattern() {
    // this is like bit operations or conjuntion using or |
    println!("# Multiple pattern");
    let x = 4;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),
        _ => println!("Anything else"),
    }
}

fn char_range_pattern() {
    println!("# Char range matching");
    let x = 'x';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn match_struct() {
    // match reach first value at struct
    let p = Point { x: 0, y: 9 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"), // this will be executed bc x = 0, first value of p Point
        Point { x, y } => println!("On neither axis: ({x}, {y})"), // This code never is execute.
    }
}

fn ignore_values_tuple() {
    println!("# Ignoring values with _");
    let values = (1, 2, 3, 4, 5);

    match values {
        (first, _, third, _, five) => {
            println!("Some values: {first}, {third} and {five}");
        }
    }
}

fn match_guard_with_or() {
    println!("# Match Guard with OR");
    let x = 4;
    let y = false;

    match x {
        // (4 | 5 | 6) if y <- El patron se comporta de esta manera.
        4 | 5 | 6 if y => println!("All is TRUE!"),
        _ => println!("Something else it's NOT TRUE!"),
    }
}

enum Message {
    Hello { id: i32 },
}

fn at_binding() {
    println!("# AT @ binding");
    let msg = Message::Hello { id: 5 };

    match msg {
        // El binding @ expone un valor para usarlo en el código de la opción del match como id_variable.
        // Esto ayuda a omitir el Shadowing como en el ejemplo anterior.
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
