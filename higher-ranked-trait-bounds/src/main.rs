fn main() {
    println!("Higher Ranked Trait Bound");
    let x = 80;
    call_with_ref(x, |y| println!("x is {}", y));

    println!(" ** Ejemplo 2 **");
    let integers = vec![2, 4, 6];
    let closure_integers = |item: &i32| println!("Numero Entero: {}", item);
    process_vector(&integers, closure_integers);
}

// Higher Ranked Trait Bound (HRTBs) ejemplo 1

// F es una función que puede tomar una referencia a T con cualquier duración (Lifetime)
// Esto permite que la función call_with_ref acepte una amplia variedad de cierres (Closures) sin tener
// problemas de duración
fn call_with_ref<T, F>(value: T, f: F)
where
    F: for<'a> FnOnce(&'a T),
{
    f(&value)
}

// Ejemplo no.2
trait Processor<T> {
    fn process(&self, item: &T);
}

// Implementar el trait en los enteros
impl Processor<i32> for () {
    fn process(&self, item: &i32) {
        println!("Procesando enteros: {}", item);
    }
}

// Funcion generica que toma un vector de elementos y un closure que los procesa
fn process_vector<T, F>(vector: &[T], f: F)
where
    T: Clone,
    F: Fn(&T),
{
    for item in vector {
        f(item);
    }
}
