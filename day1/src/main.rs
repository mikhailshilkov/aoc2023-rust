use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("./prompt.txt");

    let numbers = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let sum1: u32 = lines.iter()
        .map(|s| extract(&numbers, s))
        .sum();

    let letters = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut numbers_and_letters = HashMap::new();
    numbers_and_letters.extend(numbers);
    numbers_and_letters.extend(letters);
    let sum2: u32 = lines.iter()
        .map(|s| extract(&numbers_and_letters, s))
        .sum();
    println!("Answers: {sum1} and {sum2}");
}

fn extract(numbers: &HashMap<&str, i32>, s: &str) -> u32 {
    let first = numbers.iter()
        .map(|(k,v)| (s.find(k), v))
        .filter(|(k, _)| k.is_some())
        .map(|(k, v)| (k.expect("is some"), v))
        .min_by(|a, b| a.0.cmp(&b.0))
        .expect("at least one value")
        .1;
    let last = numbers.iter()
        .map(|(k,v)| (s.rfind(k), v))
        .filter(|(k, _)| k.is_some())
        .map(|(k, v)| (k.expect("is some"), v))
        .max_by(|a, b| a.0.cmp(&b.0))
        .expect("at least one value")
        .1;
    return format!("{first}{last}").parse().expect("must be a number");
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
