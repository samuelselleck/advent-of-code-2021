use std::fs;

fn main() { 
    let depths: Vec<f32> = fs::read_to_string("depths.txt")
        .expect("file not found.")
        .split('\n')
        .map(|s| s.parse().expect("not a number."))
        .collect();

    let depths_smooth: Vec<f32> = depths
        .windows(3)
        .map(|s| (s[0] + s[1] + s[2])/3.0)
        .collect();
    
    let increases = depths_smooth
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count();

    println!("increases: {}", increases)
}
