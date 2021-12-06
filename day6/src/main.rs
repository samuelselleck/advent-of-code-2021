use std::fs;

fn main() {
    const TOTAL_NBR_DAYS: u32 = 256;
    const BASE_TIMER: usize = 6;
    const EXTRA: usize = 2;

    let fish: Vec<usize> = fs::read_to_string("fish.txt")
        .expect("file not found.")
        .split(',')
        .map(|s| s.parse().expect("not a number."))
        .collect();

    let mut states = [0u128; BASE_TIMER + EXTRA + 1];
    for f in fish {
        states[f] += 1;
    }

    for _ in 0..TOTAL_NBR_DAYS {
        let new_fish = states[0];
        for i in 1..states.len() {
            states[i - 1] = states[i]
        }
        states[BASE_TIMER + EXTRA] = new_fish;
        states[BASE_TIMER] += new_fish;
    }

    let total: u128 = states.iter().sum();
    println!("total fish: {}", total);
}
