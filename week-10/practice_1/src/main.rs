fn main() {
    let v = vec![100, 200, 300, 400];

    // Vector v owns the object in heap

    // Only a single variable owns the heap memory
    // at any given time

    let v2 = v;

    // Here two variables own heap value,
    // two pointers to the same content is not allowed in Rust

    // Rust is very smart in terms of memory access,
    // so it detects a race condition
    // as two variables point to same heap

    println!("{:?}", v2);
}