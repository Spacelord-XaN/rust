use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1,2,3,1,2,3,3,4];
    numbers.sort();

    println!("Mean, Median and Mode of integer numbers:");
    println!("{:?}", numbers);

    println!("Mean:\t{:.3}", mean(&numbers));
    println!("Median:\t{:.3}", median(&numbers));
    println!("Mode:\t {}", mode(&numbers))
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    sum as f32 / numbers.len() as f32
}

fn median(sorted_numbers: &Vec<i32>) -> f32 {
    if sorted_numbers.len() == 1 {
        sorted_numbers[0] as f32
    }
    else if sorted_numbers.len() % 2 == 0 {
        let mut middle_numbers: Vec<i32> = Vec::new();
        middle_numbers.push(sorted_numbers[(sorted_numbers.len() - 1) / 2]);
        middle_numbers.push(sorted_numbers[((sorted_numbers.len() - 1) / 2) + 1]);
        mean(&middle_numbers)
    }
    else {
        sorted_numbers[(sorted_numbers.len() - 1) / 2] as f32
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut number_map = HashMap::new();
    let mut max_number: Option<i32> = None;
    let mut max_count: Option<i32> = None;

    for number in numbers {
        let count = number_map.entry(number).or_insert(0);
        *count += 1;

        if max_count == None && max_number == None {
            max_number = Some(*number);
            max_count = Some(*count);
        } else {
            if *count > max_count.unwrap() {
                max_number = Some(*number);
                max_count = Some(*count);
            }
        }
    }

    max_number.unwrap()
}
