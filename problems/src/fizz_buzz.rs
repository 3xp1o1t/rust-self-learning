fn divisible_by(number: u32, divisor: u32) -> bool {
    divisor != 0 && number % divisor == 0
}

fn fizz_buzz(number: u32) -> String {
    let fizz = if divisible_by(number, 3) { "fizz" } else { "" };
    let buzz = if divisible_by(number, 5) { "buzz" } else { "" };

    match (fizz.is_empty(), buzz.is_empty()) {
        (true, true) => format!("{}", number),
        _ => format!("{}{}", fizz, buzz),
    }
}

pub fn print_fizz_buzz(number: u32) {
    for i in 1..=number {
        println!("{}", fizz_buzz(i));
    }
}
