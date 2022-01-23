fn fizz_buzz(size : usize) {
    for num in 1..size {
        match (num%3, num%5) {
            (0, 0) => println!("FizzBuzz"),
            (_, 0) => println!("Buzz"),
            (0, _) => println!("Fizz"),
            (_, _) => println!("{}", num)
        }
    }
}

fn main() {
    fizz_buzz(100);
}
