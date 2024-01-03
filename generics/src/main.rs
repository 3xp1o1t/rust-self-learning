// Se require de una funcion que permita obtener el elemento mas grande de un vector
// Solo tiene que ser una funcion generica y debe implementar PartialOrd para limitar
// la comparacion a typos especificos.
// PartialOrd limita la comparacion de elementos

// FUNCION GENERICA
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_value = &list[0];

    for value in list {
        if value > largest_value {
            largest_value = value;
        }
    }

    largest_value
}

// STRUCT GENERICA
struct Point<T> {
    x: T,
    y: T,
}

struct Combined<T, U, R> {
    x: T,
    y: U,
    z: R,
}

// ENUM GENERICA
// Ejemplos existentes.
/*
* enum Option<T> {
*   Some(T),
*   None,
* }
*
* usando 2 types genericos
*
* enum Result<T, E> {
*   Ok(T),
*   Err(E),
* }
* */

// METHOD GENERICO
// Reutilizando la struct Point para implementar un meto
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    println!("Ejemplo Funcion Generica");
    let number_list = vec![10, 2, 30, 25, 9];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['x', 'y', 'e', 'a', 'z'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("Ejemplo Struct Generico<T> y combinado<T,U,R>");
    let integer_points = Point { x: 5, y: 10 };
    let float_points = Point { x: 3.14, y: 42.5 };

    println!(
        "Puntos enteros x:{}, y:{}\nPuntos flotantes: x:{}, y:{}",
        integer_points.x, integer_points.y, float_points.x, float_points.y
    );

    let num_float_bool = Combined {
        x: 42,
        y: 13.1415,
        z: true,
    };
    println!(
        "Puntos struct combinado: x num:{}, y float:{}, z bool:{}",
        num_float_bool.x, num_float_bool.y, num_float_bool.z
    );

    let point_method = Point { x: 5, y: 6 };
    println!("Method Generico {}", point_method.x());
}
