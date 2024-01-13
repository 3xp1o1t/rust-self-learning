use art::mix;
use art::PrimaryColor;

fn main() {
    println!("Generar docs");
    println!("cargo doc --open");

    println!("Using pub use for creates.");
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::yellow;
    mix(red, yellow);
}
