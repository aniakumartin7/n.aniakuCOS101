fn main() {
    // A list of numbers
    let x = vec![100, 200, 300];

    borrow_vector(&x); // Passing reference

    println!("Printing the value from main: {:?}", x[0]);
}

fn borrow_vector(z: &Vec<i32>) {
    println!("Inside print_vector function: {:?}", z);
    println!("----------------------------\n");
}