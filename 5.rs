fn main() {
    let a = 42;
    let b = 3.14;
    let c = "Hello";

    println!("Implicit types: a = {}, b = {}, c = {}", a, b, c);

    let x: i32 = 100;
    let y: f64 = 9.81;
    let z: &str = "Rust";

    println!("Explicit types: x = {}, y = {}, z = {}", x, y, z);
}

