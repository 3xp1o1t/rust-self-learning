fn main() {
    if_let_pattern();
    while_let_pattern();
    for_pattern();
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
