// Un super trait se usa para REQUERIR la funcionalidad
// de un trait dentro de otro trait.

// El trait OutlinePrint implementa el trait Display
// por lo cual cualquier instancia de OutlinePrint
// debe implementar Display ya que es requerida su funcionalidad.

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let punto = Point { x: 3, y: 6 };
    // Este call no funciona, a menos que implementos Display en Point
    punto.outline_print();
}
