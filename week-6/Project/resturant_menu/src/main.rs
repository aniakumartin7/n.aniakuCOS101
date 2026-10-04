use std::io;

fn main() {
    println!("========== RESTAURANT MENU ==========");
    println!("P - Poundo Yam / Efo Riro Soup   ₦3,200");
    println!("F - Fried Rice & Chicken         ₦3,000");
    println!("A - Amala & Ewedu Soup           ₦2,500");
    println!("E - Eba & Egusi Soup             ₦2,000");
    println!("W - White Rice & Stew             ₦2,500");
    println!("=====================================");

    // Read food type
    println!("Enter food type (P, F, A, E or W):");

    let mut food = String::new();
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read input");

    let food = food.trim().to_uppercase();

    // Read quantity
    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: f64 = quantity
        .trim()
        .parse()
        .expect("Please enter a valid quantity");

    // Determine price
    let price: f64 = match food.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // Calculate total
    let total = price * quantity;

    println!("Total before discount: ₦{:.2}", total);

    // Apply 5% discount if total is greater than ₦110,000
    if total > 110000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount (5%): ₦{:.2}", discount);
        println!("Final amount: ₦{:.2}", final_total);
    } else {
        println!("No discount applied.");
        println!("Final amount: ₦{:.2}", total);
    }
}