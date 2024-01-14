fn main() {
    let races = [
        Race{time: 40, distance: 215},
        Race{time: 92, distance: 1064},
        Race{time: 97, distance: 1505},
        Race{time: 90, distance: 1100}];
    let a1: usize = races.iter()
        .map(|r| ways_to_win(&r))
        .product();
    println!("{}", a1);

    let long_race = Race {time: 40929790, distance: 215106415051100};
    let a2 = ways_to_win(&long_race);
    println!("{}", a2);
}

fn ways_to_win(race: &Race) -> usize {
    (0..race.time)
        .map(|push| push * (race.time - push))
        .filter(|d| *d > race.distance)
        .count()
}

struct Race {
    time: u64,
    distance: u64,
}