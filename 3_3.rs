use std::io;

fn main() {
    // Read first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num1: f64 = match input1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    // Read operator
    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read operator");
    let operator = operator.trim();

    // Read second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num2: f64 = match input2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    // Perform calculation using match
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero.");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}
