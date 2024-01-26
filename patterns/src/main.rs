fn main() {
    if_let_pattern()
}

fn if_let_pattern() {
    // if let using else if let and else
    let favorite_color: Option<&str> = None;
    let is_saturday: bool = false;
    let age: Result<u8, _> = "42".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color} as the background!");
    } else if is_saturday {
        println!("Saturday best day");
    } else if let Ok(age) = age {
        if age > 28 {
            println!("Using black as background color!");
        } else {
            println!("Using red as background color!");
        }
    } else {
        println!("Using cyan as background color!");
    }
}
