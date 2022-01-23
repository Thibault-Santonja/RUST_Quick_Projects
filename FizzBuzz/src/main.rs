use std::io;


fn get_input() -> usize {
    println!("Please enter the number of iteration: ");
    let mut input= String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Bad input");

    return validate_input(input);
}

fn validate_input(text : String) -> usize {
    let trimmed = text.trim();
    match trimmed.parse::<usize>() {
        Ok(convert) => return convert,
        Err(..) => {
            println!("This is not a valid integer (need a positive integer) !");
            return get_input();
        },
    }
}

fn process_fizz_buzz(size : usize) {
    for num in 1..size+1 {
        match (num%3, num%5) {
            (0, 0) => println!("{} : FizzBuzz", num),
            (_, 0) => println!("{} : Buzz", num),
            (0, _) => println!("{} : Fizz", num),
            (_, _) => println!("{}", num)
        }
    }
}

fn main() {
    process_fizz_buzz(get_input());
}
