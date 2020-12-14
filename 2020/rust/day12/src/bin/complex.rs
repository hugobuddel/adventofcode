use num::complex::{Complex};

fn main() {
    let complex_integer = Complex::new(10, 10) + Complex::new(0, 10);
    let complex_float = Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);

    if complex_integer == num::complex::Complex::new(10, 20) {
        println!("Identical");
    }

}
