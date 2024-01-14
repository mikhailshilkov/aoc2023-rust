use std::{fs::read_to_string, collections::{HashSet, VecDeque}};

fn main() {
    let lines = read_lines("./prompt.txt");
    let all_cards: Vec<Card> = lines.iter()
        .map(|l| parse_card(l))
        .collect();

    let a1: u32 = all_cards.iter()
        .map(|c| winning_subset(&c))
        .map(|n|score(n))
        .sum();
    println!("{}", a1);

    let a2 = process_cards(all_cards);
    println!("{}", a2);
}

fn process_cards(initial: Vec<Card>) -> u32 {
    let mut cards: VecDeque<(&Card, u32)> = initial.iter().map(|c|(c, 1)).collect::<VecDeque<_>>();
    let mut count = 0;
    while !cards.is_empty() {
        let (card, num) = cards.pop_front().expect("first card");
        let winning_len = winning_subset(card);
        for i in 0..winning_len {
            cards[i] = (cards[i].0, cards[i].1 + num);
        }
        count += num;
    }
    count
}

fn score(n: usize) -> u32 {
    if n == 0 { return 0 }
    let base: u32 = 2; // an explicit type is required
    base.pow(u32::try_from(n-1).expect("usize"))
}

fn winning_subset(c: &Card) -> usize {
    let mut winning_set = HashSet::new();
    for n in c.winning_numbers.iter() {
        winning_set.insert(n);
    }
    c.your_numbers.iter().filter(|n| winning_set.contains(n)).count()
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

#[derive(Debug)]
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
