fn main() {
    let p: f64 = 210_000.0;
    let r: f64 = 5.0;
    let n: i32 = 3;

    let value = p * (1.0 - r / 100.0).powi(n);

    println!("Original Value = ₦210000", p);
    println!("Depreciation Rate = 5%", r);
    println!("Age = 3 years", n);
    println!("Value after 3 years = ₦181578.75", n, value);
}