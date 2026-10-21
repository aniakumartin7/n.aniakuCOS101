use std::io::Write;

// The Categories of drinks
fn main(){
    let mut file = std::fs::File::create("nigerian_brewies.csv").expect("create failed");
    // The header
    let header = "Larger, Stout, Non-Alcoholic";
    file.write_all(header.as_bytes()).expect("Failed to create header");
    // Row one
    let row_1 = "\n33 Export, Legend, Maltina";
    file.write_all(row_1.as_bytes()).expect("failed to create row 1");
    // Row two
    let row_2 = "\nDesperados, Turbo King, Amstel Malta";
    file.write_all(row_2.as_bytes()).expect("failed to create row 2");
    // Row three
    let row_3 = "\nGoldberg, Williams, Malta Gold";
    file.write_all(row_3.as_bytes()).expect("failed to create row 3");
    // Row four
    let row_4 = "\nGulder,,Fayrouz";
    file.write_all(row_4.as_bytes()).expect("failed to create row 4");
    // Row 5
    let row_5 = "\nHeineken,,";
    file.write_all(row_5.as_bytes()).expect("failed to create row 5");
    // Row 6
    let row_6 = "\nStar,,";
    file.write_all(row_6.as_bytes()).expect("failed to create row 6");

    println!("Nigerian Brewies data is saved");
}