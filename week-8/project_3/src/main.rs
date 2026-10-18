use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daodu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc.txt").unwrap();

    for i in 0..commissioners.len() {
        println!("Commissioner: {}", commissioners[i]);
        println!("Ministry: {}", ministries[i]);
        println!("Zone: {}", zones[i]);

        file.write_all(
            format!(
                "Commissioner: {}\nMinistry: {}\nZone: {}\n\n",
                commissioners[i], ministries[i], zones[i]
            )
            .as_bytes()
        ).unwrap();
    }

    println!("EFCC records saved successfully");
}