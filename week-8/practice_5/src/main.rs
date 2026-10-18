use std::io;

fn main() {
    // Create an empty vector of cities
    let mut city: Vec<String> = Vec::new();

    println!("The City vector has element {}", city.len());

    // Ask how many cities the user wants to enter
    let mut input1 = String::new();

    println!("How many cities do you want to enter?");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    // Add cities to the vector
    for count in 0..city_num {
        let mut input2 = String::new();

        println!("Enter City {}", count + 1);
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");

        let new_city: String = input2.trim().to_string();
        city.push(new_city);
    }

    // Display all the cities
    println!("Your preferred cities are:");

    let mut count = 1;

    for i in &city {
        println!("{}: {}", count, i);
        count += 1;
    }
}