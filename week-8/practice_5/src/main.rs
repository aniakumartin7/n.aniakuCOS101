
use std::io;

fn main() {
    // Create an empty vector called city
    let mut city: Vec<String> = Vec::new();

    // Print the city vector length
    println!("The City vector has {} element(s)", city.len());

    // Push new elements into the vector
    let mut input1 = String::new();

    println!("How many cities do you want to enter?");

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    let count: usize = input1.trim().parse().expect("Invalid input");

    for i in 0..count {
        let mut input2 = String::new();

        println!("Enter city {}:", i + 1);

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");

        let new_city: String = input2.trim().to_string();

        city.push(new_city);
    }

    println!("Your preferred cities are:");

    let mut count = 1;

    // Loop to display elements in the vector
    for i in city.iter() {
        println!("{}: {}", count, i);
        count += 1;
    }
}
