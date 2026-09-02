	fn main() {
		let p:f64 = 1000.0;
		let r:f64 = 1.0;
		let _t:f64 = 2.0;

		// Amount = P * (1 + r / 100)
		let a = p * ( 1.0 + (r / 100.0));

		let si = a - p;

		println!("Amount is {}", a);
		println!("Simple Interest is {}", si);

	}