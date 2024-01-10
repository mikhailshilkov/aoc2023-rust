use std::{fs::read_to_string, collections::HashSet};

fn main() {
    let lines = read_lines("./prompt.txt");
    let cards: u32 = lines.iter()
        .map(|l| parse_card(l))
        .map(|c| winning_subset(c))
        .map(|v|v.len())
        .map(|n|score(n))
        .sum();
    println!("{:?}", cards);
}

fn score(n: usize) -> u32 {
    if n == 0 { return 0 }
    let base: u32 = 2; // an explicit type is required
    base.pow(u32::try_from(n-1).expect("usize"))
}

fn winning_subset(c: Card) -> Vec<u32> {
    let mut winning_set = HashSet::new();
    for n in c.winning_numbers.iter() {
        winning_set.insert(n);
    }
    c.your_numbers.into_iter().filter(|n| winning_set.contains(n)).collect()
}

fn parse_card(l: &str) -> Card {
    let parts: Vec<&str> = l.split(": ").collect();
    let parts: Vec<&str> = parts[1].split(" | ").collect();
    fn parse_numbers(s: &str) -> Vec<u32> {
        s.split(" ").filter(|s| s.len() > 0).map(|s| s.parse().expect("number")).collect()
    }
    Card {
        winning_numbers: parse_numbers(parts[0]),
        your_numbers: parse_numbers(parts[1]),
    }
}

struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
