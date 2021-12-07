use std::fs;

fn main() {
    let crab_pos: Vec<i32> = fs::read_to_string("crabs.txt")
        .expect("file not found.")
        .split(',')
        .map(|s| s.parse().expect("not a number."))
        .collect();

    let &max = crab_pos.iter().max().unwrap();
    let &min = crab_pos.iter().min().unwrap();

    let min_fuel: i32 = (min..=max)
        .map(|h| crab_pos.iter().map(|c| {
            let n = (c - h).abs();
            n*(n + 1)/2
        }).sum())
        .min().unwrap();
    
    println!("fuel: {}", min_fuel);
}
