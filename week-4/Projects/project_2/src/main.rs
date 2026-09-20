use std::io;

fn main() {
    let mut input = String::new();

    println!("Are you experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experienced = input.trim().to_lowercase();

    input.clear();

    println!("Enter your age:");
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

    if experienced == "yes" {
        if age >= 40 {
            println!("Annual incentive: ₦1,560,000");
        } else if age >= 30 && age <= 38 {
            println!("Annual incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual incentive: ₦1,300,000");
        } else {
            println!("No incentive specified for this age range.");
        }
    } else {
        println!("Annual incentive: ₦100,000");
    }
}