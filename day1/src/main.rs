use std::fs;

fn main() { 
    let depths: Vec<_> = fs::read_to_string("depths.txt")
        .expect("file not found.")
        .split('\n')
        .map(|s| s.parse().expect("not a number."))
        .collect();

    let depths_smooth: Vec<_> = depths
        .windows(3)
        .map(|s| s.iter().sum::<f32>()/3.0)
        .collect();
    
    let increases = depths_smooth
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count();

    println!("increases: {}", increases)
}
