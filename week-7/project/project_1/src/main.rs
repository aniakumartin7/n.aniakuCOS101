use std::io;

fn main() {
    println!("Shape Calculator");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice: i32 = choice.trim().parse().unwrap();

    match choice {
        1 => trapezium(),
        2 => rhombus(),
        3 => parallelogram(),
        4 => cube(),
        5 => cylinder(),
        _ => println!("Invalid choice"),
    }
}

fn trapezium() {
    println!("Enter height:");
    let height = get_number();

    println!("Enter base 1:");
    let base1 = get_number();

    println!("Enter base 2:");
    let base2 = get_number();

    let area = height / 2.0 * (base1 + base2);

    println!("Area of trapezium is {}", area);
}

fn rhombus() {
    println!("Enter diagonal 1:");
    let diagonal1 = get_number();

    println!("Enter diagonal 2:");
    let diagonal2 = get_number();

    let area = 0.5 * diagonal1 * diagonal2;

    println!("Area of rhombus is {}", area);
}

fn parallelogram() {
    println!("Enter base:");
    let base = get_number();

    println!("Enter altitude:");
    let altitude = get_number();

    let area = base * altitude;

    println!("Area of parallelogram is {}", area);
}

fn cube() {
    println!("Enter side:");
    let side = get_number();

    let area = 6.0 * side * side;

    println!("Surface area of cube is {}", area);
}

fn cylinder() {
    println!("Enter radius:");
    let radius = get_number();

    println!("Enter height:");
    let height = get_number();

    let volume = 3.14159 * radius * radius * height;

    println!("Volume of cylinder is {}", volume);
}

fn get_number() -> f64 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let number: f64 = input.trim().parse().unwrap();

    number
}