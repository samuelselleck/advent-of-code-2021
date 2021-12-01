use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    
    let depths = load_from_file("depths.txt");
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

fn load_from_file(file_path: &str) -> Vec<f32> {
    let file = File::open(file_path).expect("file wasn't found.");
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}
