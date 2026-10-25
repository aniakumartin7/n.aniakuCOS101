use std::io;
use std::io::Write;
//PAU-SMIS
fn main() {
    let mut File = std::fs::File::create("pau-smis.csv").expect("failed to create file");
    let header = "\nStudent name, Matric Number, Department, Level";
    File.write_all(header.as_bytes()).expect("Failed to create the header");
    println!("Put the amount of students to be entered");
    let mut amt = String::new();
    io::stdin().read_line(&mut amt).expect("Failed to read input");
    let amt: u16 = amt.trim().parse().expect("Invalid input");
    let mut vname: Vec<String> = Vec::new();
    let mut vmatric_no: Vec<String> = Vec::new();
    let mut vdept: Vec<String> = Vec::new();
    let mut vlevel: Vec<u16> = Vec::new();

for i in 0..amt as usize{
    println!("\nInformation for student {}", i + 1);
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string();
    vname.push(name);

     let mut matric_no = String::new();
    io::stdin().read_line(&mut matric_no).expect("Failed to read input");
    let matric_no = matric_no.trim().to_string();
    vmatric_no.push(matric_no);

     let mut dept = String::new();
    io::stdin().read_line(&mut dept).expect("Failed to read input");
    let dept = dept.trim().to_string();
    vdept.push(dept);

     let mut level = String::new();
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let level: u16 = level.trim().parse().expect("Failed to read input");
    vlevel.push(level);
    // format allows me to get data from the vectors and put them as strings.It enables me to plugin the strings from my vectors to the placeholders
    let row = format!("\n{},{},{},{}", vname[i], vmatric_no[i], vdept[i], vlevel[i]);
    File.write_all(row.as_bytes()).expect("failed to create row")
}
println!("Data has been written to file")
}
