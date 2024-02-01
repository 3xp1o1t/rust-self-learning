trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your pilot speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up wizard!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously* humans...");
    }
}
fn main() {
    // sintaxis completamente calificada para llamar a métodos con el mismo nombre
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // sintaxis no completamente calificada
    let person = Human;
    // Por defecto el método implementado directamente en el struct es llamado.
    person.fly();
}

// trait Human: Pilot + Wizard {
//     fn fly(&self);
// }

// El formato de la sintaxis completamente calificada es:
// impl <trait> for <type> {
//     fn <method>(&self) {
//         // code
//     }
// }
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
