use std::io;

fn main() {
    println!("Pig-latin");
    println!("Enter a word to convert it to pig-latin:");

    let mut word = String::new();
    io::stdin()
        .read_line(& mut word)
        .expect("Failed to read line");

    let result = pig_latin(word.trim());
    println!("{}", result);
}

fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first = chars.next();

    match first {
        Some(letter) => {
            if is_vowel(letter) {
                format!("{}-{}", word, "hay")
            }
            else {
                let first_word = word.chars().skip(1).collect::<String>();
                format!("{}-{}{}", first_word, letter.to_lowercase(), "ay")
            }
        }
        None => {
            String::new()
        }
    }
}

fn is_vowel(letter: char) -> bool {
    match letter {
        'a' => true,
        'A' => true,
        'e' => true,
        'E' => true,
        'i' => true,
        'I' => true,
        'o' => true,
        'O' => true,
        'u' => true,
        'U' => true,
        _ => false
    }
}
