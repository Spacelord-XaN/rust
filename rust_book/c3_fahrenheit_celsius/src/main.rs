use std::io;

fn main() {
    println!("Fahrenheit <=> Celsius");
    println!("Choose direction:");
    println!("  (1) Celsius    ==> Fahrenheit");
    println!("  (2) Fahrenheit ==> Celsius");

    let mut direction = String::new();

    io::stdin()
        .read_line(&mut direction)
        .expect("Failed to read line");

    match direction.trim() {
        "1" => {
            celsius_to_fahrenheit();
        }
        "2" => {
            fahrenheit_to_celsius();
        }
        _ => {
            println!("Invalid direction.");
        }
    };
}

fn celsius_to_fahrenheit() {
    println!("Enter degrees in Celsius:");
    
    match input_number() {
        Some(celsius) => {
            let fahrenheit = (celsius * 1.8) + 32.0;
            println!("Fahrenheit: {:.2}", fahrenheit);
        },
        None => {}
    }
}

fn fahrenheit_to_celsius() {
     println!("Enter degrees in Fahrenheit:");

     match input_number() {
         Some(fahrenheit) => {
             let celsius = (fahrenheit -32.0) * 5.0 / 9.0;
             println!("Celsius: {:.2}", celsius);
         },
         None => {}
     }
}

fn input_number() -> Option<f32> {
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
