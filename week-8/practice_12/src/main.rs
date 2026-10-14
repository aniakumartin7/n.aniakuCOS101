fn main() {
    // Mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // Create a mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // Change the value at index 1 of the slice
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}