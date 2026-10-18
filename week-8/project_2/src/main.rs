use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE10102020", "Electrical", 200),
        ("Bianca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = File::create("students.txt").unwrap();

    for student in students.iter() {
        println!("Student Name: {}", student.0);
        println!("Matric Number: {}", student.1);
        println!("Department: {}", student.2);
        println!("Level: {}", student.3);

        file.write_all(
            format!(
                "Student Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n\n",
                student.0, student.1, student.2, student.3
            )
            .as_bytes()
        ).unwrap();
    }

    println!("Student details saved successfully");
}