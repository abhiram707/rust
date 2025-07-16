use std::io;

fn main() {
    println!("Enter an integer:");

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse input to integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };

    // Check if the number is even or odd
    if number % 2 == 0 {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
}
