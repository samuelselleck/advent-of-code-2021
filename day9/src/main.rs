use std::fs;
use std::collections::HashSet;

fn main() {
    let map: Vec<Vec<u8>> = fs::read_to_string("map_t.txt")
        .expect("file not found.")
        .split('\n')
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    
    let h = |i, j| if i < 0 || j < 0 { None } else { map.get(i as usize).and_then(|v: &Vec<_>| v.get(j as usize)) };

    let mut low_points = vec![];
    for i in 0..map.len() as i32 {
        for j in 0..map[0].len() as i32 {
            let curr = h(i, j).unwrap();
            let surr = [h(i + 1, j), h(i - 1, j), h(i, j + 1), h(i, j - 1)];
            let low_point = surr.iter().filter_map(|&h| h).all(|v| v > curr);
            if low_point {
                low_points.push((i, j));
            }
        }
    }

    let mut visited = HashSet::new();
    let mut neighbors = vec![];

    for low_point in low_points {
        neighbors.push(low_point);
        while neighbors.len() > 0 {
            let pos = neighbors.pop();
            
        }
    }

    println!("{:?}", low_points);
}
