use std::{fs::File, io::ErrorKind, num::ParseIntError};

fn main() {
    recuperable_error().unwrap();
    // open_file()
    file_open_clousure()
}

// Errores recuperables con Result
fn recuperable_error() -> Result<(), ParseIntError> {
    // catch error if not a number
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    println!("{}", number);
    Ok(())
}

// Manejando diferentes tipos de errores.
fn open_file() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Creating the file hello.txt");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };
}

// Version mejorara de la funcion de arriba.
// La funcion de arriba, usa muchos match.
// Es posible mejorarla usando Clousures y metodo unwrap_or_else;
fn file_open_clousure() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}
