use std::io;

fn main() {
    println!("Fibonacci");
    println!("Generates the nth fibonacci number.");
    println!("Please enter a number:");

    match input_number() {
        Some(number) => {
            let result = fibonacci(number - 1);
            println!("{}", result);
        }
        None => {}
    }
}

fn fibonacci(number: i32) -> i32 {
    if number == 0 || number == 1 {
        number
    }
    else {
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}

fn input_number() -> Option<i32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Please enter a number.");
            None
        }
    }
}
