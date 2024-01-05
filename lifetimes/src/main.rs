use std::fmt;

// Lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime en struct
struct ImportanExcerpt<'z> {
    part: &'z str,
}

// lifetime en metodo de struct
impl<'z> fmt::Display for ImportanExcerpt<'z> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.part)
    }
}

// Ejemplo usando Genericos, traitbounds y lifetimes juntos
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    //Ejemplo usando lifetime
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Ejemplo usando struct y lifetime

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportanExcerpt {
        part: first_sentence,
    };

    println!("First sentence: {}", i);

    // Ejemplo usando genericos, lifetimes y trait bounds
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_announcement(string1.as_str(), string2, "Today is a great day!");
    println!("The longest string is {result}");
}
