fn main() {
    let developers = vec![
        ("John", 3),
        ("Mary", 7),
        ("Peter", 5),
        ("David", 10),
        ("Grace", 4),
    ];

    let mut highest_name = developers[0].0;
    let mut highest_years = developers[0].1;

    for developer in developers.iter() {
        if developer.1 > highest_years {
            highest_years = developer.1;
            highest_name = developer.0;
        }
    }

    println!("Developer with the highest experience:");
    println!("Name: {}", highest_name);
    println!("Years of experience: {}", highest_years);
}