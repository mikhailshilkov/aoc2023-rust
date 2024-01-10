use std::{fs::read_to_string, cmp};

fn main() {
    let lines = read_lines("./prompt.txt");
    let a1: u32 = lines.iter()
        .enumerate()
        .flat_map(|(row, s)| find_numbers(row, s))
        .filter(|n| has_symbol(n, &lines))
        .map(|n| n.num)
        .sum();
    println!("{}", a1);

    let a2: u32 = lines.iter()
        .enumerate()
        .flat_map(|(row, s)| find_stars(row, s))
        .filter_map(|s| to_gear(&s, &lines))
        .map(|g| g.0*g.1)
        .sum();
    println!("{}", a2);
}

fn find_stars(x: usize, s: &str) -> Vec<Star> {
    s.chars()
        .enumerate()
        .filter(|(_,c)| *c == '*')
        .map(|(i, _)| Star{x: x, y: i})
        .collect()
}

fn to_gear(star: &Star, lines: &Vec<String>) -> Option<Gear> {
    let min_x = if star.x > 0 { star.x - 1} else { 0 };
    let min_y = if star.y > 0 { star.y - 1} else { 0 };
    let max_x = cmp::min(star.x + 1, lines.len() - 1);
    let max_y = cmp::min(star.y + 1, lines[star.x].len() - 1);

    let mut digits = Vec::new();
    for x in min_x..max_x + 1 {
        let line = &lines[x];
        let mut prev_digit = false;
        for y in min_y..max_y + 1 {
            let c = line.chars().nth(y).expect("char");
            if c.is_ascii_digit() {
                if !prev_digit {
                    digits.push((x, y));
                    prev_digit = true
                }
            } else {
                prev_digit = false
            }
        }
    }

    match digits.len() {
        x if x > 2 => panic!("Found 3+ numbers somehow, must be a bug"),
        x if x < 2 => Option::None, 
        _ => {
            let n1 = parse_digit(digits[0].1, &lines[digits[0].0]);
            let n2 = parse_digit(digits[1].1, &lines[digits[1].0]);
        Option::Some(Gear(n1, n2))
        }
    }
}

fn parse_digit(y: usize, line: &str) -> u32 {
    let mut min_y = y;
    while min_y > 0 && line.chars().nth(min_y-1).expect("digit").is_ascii_digit() {
        min_y-=1;
    }
    let mut max_y = y;
    while max_y < line.len() - 1 && line.chars().nth(max_y+1).expect("digit").is_ascii_digit() {
        max_y+=1;
    }
    let sub = &line[min_y..max_y+1];
    sub.parse().expect("number")
}

struct Star {
    x: usize,
    y: usize,
}

struct Gear(u32, u32);

struct Number {
    num: u32,
    x: usize,
    y: usize,
}

fn has_symbol(n: &Number, lines: &Vec<String>) -> bool {
    let min_x = if n.x > 0 { n.x - 1} else { 0 };
    let min_y = if n.y > 0 { n.y - 1} else { 0 };
    let max_x = cmp::min(n.x + 1, lines.len() - 1);
    let max_y = cmp::min(n.y + n.num.to_string().len(), lines[n.x].len() - 1);
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let c = lines[x].chars().nth(y).expect("char");
            if !c.is_ascii_digit() && c != '.' {
                return true
            }
        }
    }
    false
}

fn find_numbers(x: usize, s: &str) -> Vec<Number> {
    let mut res = Vec::new();
    let mut buf = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            buf.push(c);
        } else if !buf.is_empty() {
            res.push(Number {
                x,
                y: i - buf.len(),
                num: buf.parse().expect("number"),
            });
            buf.clear();
        }
    }
    if !buf.is_empty() {
        res.push(Number {
            x,
            y: s.len() - buf.len(),
            num: buf.parse().expect("number"),
        });
    }
    res
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
