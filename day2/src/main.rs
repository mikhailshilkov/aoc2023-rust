use core::panic;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./prompt.txt");
    let games: Vec<Game> = lines.iter()
        .map(|l| parse_game(l))
        .collect();
    let a1: u32 = games.iter()
        .filter(|g| possible_game(g))
        .map(|g| g.index)
        .sum();
    println!("{:?}", a1);

    let a2: u32 = games.iter()
        .map(|g|smallest_draw(g))
        .map(|d|d.blue*d.red*d.green)
        .sum();
    println!("{:?}", a2);
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    index: u32,
    draws: Vec<Draw>,
}

fn possible_game(g: &Game) -> bool {
    g.draws.iter().all(|d| d.blue <= 14 && d.green <= 13 && d.red <= 12)
}

fn smallest_draw(g: &Game) -> Draw {
    let max_red = g.draws.iter().map(|d|d.red).max().unwrap_or(0);
    let max_green = g.draws.iter().map(|d|d.green).max().unwrap_or(0);
    let max_blue = g.draws.iter().map(|d|d.blue).max().unwrap_or(0);
    Draw {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn parse_game(l: &str) -> Game {
    let parts: Vec<&str> = l.split(": ").collect();
    let index = parts[0].replace("Game ", "").parse().expect("game number");
    let draws = parts[1].split("; ").map(|s| parse_draw(s)).collect();
    Game {
        index: index,
        draws: draws,
    }
}

fn parse_draw(s: &str) -> Draw {
    let parts: Vec<&str> = s.split(", ").collect();
    let mut green = 0;
    let mut blue = 0;
    let mut red = 0;
    for part in parts {
        let n = part.split(" ").next().and_then(|s| s.parse::<u32>().ok()).expect("number of parts");
        if part.contains("blue") {
            blue = n;
        } else if part.contains("red") {
            red = n;
        } else if part.contains("green") {
            green = n;
        } else {
            panic!("unknown color {}", part)
        }
    }
    Draw {
        green: green,
        blue: blue,
        red: red,
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
