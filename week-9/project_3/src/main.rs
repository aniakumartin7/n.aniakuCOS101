use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("efcc-merge.csv").expect("Failed to create file");
    file.write_all("Ministers convicted by EFCC".as_bytes()).expect("failed to write");

    let header = "Commisioner, Ministry, Zone\n";
    file.write_all(header.as_bytes()).expect("failed to create table");
    
    let names = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa\n"];

    let ministry = vec!["Internal Affairs","Justice","Defence","Power & Steel","Petroleum\n"];

    let zone = vec!["South West","North East","South South","South West","South East\n"];

    for i in 0..names.len() {
        let column = format!("{},{},{}\n",names[i],ministry[i], zone[i]);

        file.write_all(column.as_bytes()).expect("write failed"); }

    println!("Data written to file");
}