fn main() {
    let p: f64 = 520_000_000.0;
    let r: f64 = 10.0;
    let n: i32 = 5;

    let a = p * (1.0 + r / 100.0).powi(n);
    let ci = a - p;

    println!("Principal = ₦520000000", p);
    println!("Rate = 10%", r);
    println!("Time = 5 years", n);
    println!("Amount = ₦837465200", a);
    println!("Compound Interest = ₦317465200", ci);
}