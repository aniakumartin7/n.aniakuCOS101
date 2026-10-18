use std::fs::File;
use std::io::Write;

fn main() {
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("drinks.txt").expect("Unable to create file");

    writeln!(file, "NIGERIAN BREWERIES").unwrap();
    writeln!(file, "").unwrap();

    writeln!(file, "Lager").unwrap();
    for drink in lager.iter() {
        writeln!(file, "{}", drink).unwrap();
    }

    writeln!(file, "").unwrap();
    writeln!(file, "Stout").unwrap();
    for drink in stout.iter() {
        writeln!(file, "{}", drink).unwrap();
    }

    writeln!(file, "").unwrap();
    writeln!(file, "Non-Alcoholic").unwrap();
    for drink in non_alcoholic.iter() {
        writeln!(file, "{}", drink).unwrap();
    }

    println!("Drinks have been saved to drinks.txt");
}