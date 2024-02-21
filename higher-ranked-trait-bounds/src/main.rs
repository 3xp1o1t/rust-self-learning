fn main() {
    println!("Higher Ranked Trait Bound");
    let x = 80;
    call_with_ref(x, |y| println!("x is {}", y));
}

// Higher Ranked Trait Bound (HRTBs) ejemplo

// F es una función que puede tomar una referencia a T con cualquier duración (Lifetime)
// Esto permite que la función call_with_ref acepte una amplia variedad de cierres (Closures) sin tener
// problemas de duración
fn call_with_ref<T, F>(value: T, f: F)
where
    F: for<'a> FnOnce(&'a T),
{
    f(&value)
}
