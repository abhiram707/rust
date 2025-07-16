use std::io;

fn main() {
    // Prompt user
    println!("Enter a number:");

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse input to number
    let number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };

    // Check if the number is positive, negative, or zero
    if number > 0.0 {
        println!("The number is positive.");
    } else if number < 0.0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}
