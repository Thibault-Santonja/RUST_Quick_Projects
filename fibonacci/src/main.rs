// Quicker Fibonacci
pub fn fibonacci(n: i32) -> i128 {
	if n < 0 {
		panic!("{} is negative!", n)
	}
	return fast_fib(n).0;
}

pub fn fast_fib(n: i32) -> (i128, i128) {
	if n == 0 {
		return (0, 1)
	}
	let (a, b) = fast_fib(n / 2);
	let c = a * (b * 2 - a);
	let d = a * a + b * b;
	return if n % 2 == 0 {
		(c, d)
	} else {
		(d, c + d)
	}
}

// Main
fn main() {
    println!("Hello World!");

	for i in 0..11 {
    	println!("Fib {} : {}", i, fibonacci(i));
	}

	for i in (40..200).step_by(40) {
    	println!("Fib {} : {}", i, fibonacci(i));
	}


	let mut count = 0;
	'counting_up: loop {
		println!("count = {}", count);
		let mut remaining = 10;

		loop {
			println!("remaining = {}", remaining);
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}
	println!("End count = {}", count);

	let a = [10, 20, 30, 40, 50];

	for element in a {
		println!("the value is: {}", element);
	}

	for number in (1..4).rev() {
		println!("{}!", number);
	}
	println!("LIFTOFF!!!");
}
