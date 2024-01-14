use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines = read_lines("./prompt.txt");
    let almanac = read_almanac(lines);
    let min_location1 = almanac.seeds.iter()
        .map(|s| item_location("seed", *s, 1, &almanac))
        .min().expect("min");
    println!("{}", min_location1);

    let min_location2 = almanac.seeds
        .chunks(2)
        .map(|c| item_location("seed", c[0], c[1], &almanac))
        .min().expect("min");
    println!("{}", min_location2);
}

fn item_location(category: &str, start: u64, length: u64, almanac: &Almanac) -> u64 {
    let map_name = almanac.categories.iter()
        .filter(|c| c.starts_with(category))
        .next().expect("category");
    let out_category = map_name.split("-to-").nth(1).expect("out category");
    let map = &almanac.mappings[map_name];
    let output = map_in_range(start, length, map);
    match out_category {
        "location" => *output.iter().map(|(start, _)| start).min().expect("min"),
        c => 
            output.iter()
                .map(|(s, l)| item_location(c, *s, *l, almanac))
                .min().expect("min"),
    }
}

fn map_in_range(item: u64, length: u64, ranges: &Vec<Range>) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    let mut current_item = item;
    let mut current_length = length;
    for range in ranges {
        if current_item >= range.source_start && current_item + current_length <= range.source_start + range.length {
            // Entirely within this range.
            current_item = (current_item - range.source_start) + range.destination_start;
            break;
        } else if current_item >= range.source_start && current_item < range.source_start + range.length {
            // Start is within the range but end is beyond its end.
            let start = (current_item - range.source_start) + range.destination_start;
            let end = range.destination_start + range.length;
            res.push((start, end-start));
            current_length = current_item + current_length - (range.source_start + range.length);
            current_item = range.source_start + range.length;
        } else if current_item + current_length > range.source_start && current_item + current_length <= range.source_start + range.length {
            // Start is before the range but end is within.
            let start = range.destination_start;
            let end = current_item + current_length + range.destination_start - range.source_start;
            res.push((start, end-start));
            current_length = range.source_start - current_item;
        }
    }
    res.push((current_item, current_length));
    res
}

fn read_almanac(lines: Vec<String>) -> Almanac {
    let seeds: Vec<u64> = lines[0]
        .strip_prefix("seeds: ").expect("seeds")
        .split(" ")
        .map(|s|s.parse().expect("seed"))
        .collect();

    let mut mappings: HashMap<String, Vec<Range>> = HashMap::new();
    let mut categories: Vec<String> = Vec::new();
    let mut category = "";
    let mut ranges: Vec<Range> = Vec::new();
    for i in 1..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            if category != "" {
                mappings.insert(category.to_string(), ranges);
            }
            category = "";
            ranges = Vec::new();
        } else if line.ends_with(" map:") {
            category = line.strip_suffix(" map:").expect("map");
            categories.push(category.to_string());
        } else if category != "" {
            ranges.push(parse_range(&line));
        }
    }
    mappings.insert(category.to_string(), ranges);
    
    Almanac {
        seeds,
        categories,
        mappings,
    }
}

fn parse_range(s: &str) -> Range {
    let nums: Vec<u64> = s.split(" ").map(|s| s.parse().expect("range number")).collect();
    if nums.len() != 3 {
        panic!("range {}", s);
    }
    Range {
        destination_start: nums[0],
        source_start: nums[1],
        length: nums[2],
    }
}

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    categories: Vec<String>,
    mappings: HashMap<String, Vec<Range>>,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
