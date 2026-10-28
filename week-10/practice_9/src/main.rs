// Define dimensions of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        // Use self to access the fields
        self.width * self.height
    }
}

fn main() {
    // Instantiate the structure
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    // Print the rectangle's area
    println!(
        "Width is {} \nHeight is {} \nArea of Rectangle is {}",
        small.width,
        small.height,
        small.area()
    );
}