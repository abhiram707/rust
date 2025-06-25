fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice_a = &arr[1..3];
    let slice_b = &arr[..3];
    let slice_c = &arr[5..];
    let slice_d = &arr[..];

    println!("Slice a (2nd and 3rd element): {:?}", slice_a);
    println!("Slice b (omit start index): {:?}", slice_b);
    println!("Slice c (omit end index): {:?}", slice_c);
    println!("Slice d (omit both): {:?}", slice_d);
}

