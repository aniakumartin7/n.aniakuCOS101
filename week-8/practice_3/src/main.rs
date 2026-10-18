
fn value(n: Option<char>) {
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['C', 'O', 'S', 'T', 'A', 'C', '1', 'A', 'N'];

    let mut input1 = String::new();

    println!("Enter an index value between 0 - 8");

    std::io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    // Convert input to an index
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Getting value at given index
    let ch: Option<char> = v.get(index).copied();

    value(ch);
}
