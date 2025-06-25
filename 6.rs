fn main() {
    let arr: [i32; 6] = [0, 2, 4, 6, 8, 10];

    println!("The array contains:");
    for value in arr.iter() {
        println!("{}", value);
    }
}

