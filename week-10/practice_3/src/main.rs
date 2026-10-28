fn main() {
    let v = vec![20, 40, 60, 80];

    // Vector v owns the object in the heap

    let v2 = v;

    let v2_return = display(v2);

    println!("In main (v): {:?}", v2_return);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    // Returning the same vector
    println!("Inside display: {:?}", v);

    return v;
}