use std::io;

struct Laptop {
    brand: String,
    quantity: u32,
    price: f64,
}

impl Laptop {
    fn cost(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

fn main() {
    let mut laptops = vec![
        Laptop { brand: String::from("HP"), quantity: 3, price: 650_000.0 },
        Laptop { brand: String::from("IBM"), quantity: 3, price: 755_000.0 },
        Laptop { brand: String::from("Toshiba"), quantity: 3, price: 550_000.0 },
        Laptop { brand: String::from("Dell"), quantity: 3, price: 850_000.0 },
    ];

    for laptop in &mut laptops {
        println!("Enter quantity for {} (3 only):", laptop.brand);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let mut quantity: u32 = input.trim().parse().expect("Failed to read");

        while quantity < 3 {
            println!("You have to buy at least 3 laptops from each brand. Enter quantity again:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            quantity = input.trim().parse().expect("Failed to read");
        }

        laptop.quantity = quantity;
    }

    let mut total = 0.0;
    println!("\nTotal Purchase");

    for laptop in &laptops {
        let amount = laptop.cost();

        println!("Brand: {}", laptop.brand);
        println!("Quantity: {}", laptop.quantity);
        println!("Unit price: ₦{:.2}", laptop.price);
        println!("Total cost: ₦{:.2}\n", amount);

        total += amount;
    }

    println!("Total amount spent: ₦{:.2}", total);
}