fn main() {
    let v = vec![10, 20, 30];

    // Move ownership from v to v2
    let v2 = v;

    display(v2);

    // v and v2 cannot be used here because ownership has moved
}

fn display(v: Vec<i32>) {
    println!("Inside display: {:?}", v);
}