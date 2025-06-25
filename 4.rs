fn main() {
    let x = 5;
    println!("Outside inner scope, x = {}", x);

    {
        let x = 10;
        println!("Inside inner scope, shadowed x = {}", x);

        let y = 20;
        println!("Inside inner scope, y = {}", y);
    }

    println!("Back to outer scope, x = {}", x);
}

