fn main() {
    let b: (i32, bool, f64) = (110, true, 10.9);
    print_tuple(b);
}

fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}