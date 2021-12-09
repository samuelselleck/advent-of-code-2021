use std::collections::HashSet;
use std::fs;

struct Map {
    terrain: Vec<Vec<u8>>,
    dims: (i32, i32),
}

impl Map {
    fn new(terrain: Vec<Vec<u8>>) -> Map {
        Map {
            dims: (terrain.len() as i32, terrain[0].len() as i32),
            terrain,
        }
    }
    fn get_height(&self, i: i32, j: i32) -> Option<&u8> {
        self.terrain
            .get(i as usize)
            .and_then(|v: &Vec<_>| v.get(j as usize))
    }

    fn neighborhood(&self, i: i32, j: i32) -> impl Iterator<Item = ((i32, i32), u8)> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(move |(di, dj)| (i + di, j + dj))
            .filter_map(move |(i, j)| self.get_height(i, j).and_then(|h| Some(((i, j), *h))))
    }
}

fn main() {
    let map = Map::new(
        fs::read_to_string("map.txt")
            .expect("file not found.")
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    );

    let mut low_points = vec![];
    let (w, h) = map.dims;
    for i in 0..w {
        for j in 0..h {
            let curr = map.get_height(i, j).unwrap();
            if map.neighborhood(i, j).all(|(_, h)| h > *curr) {
                low_points.push((i, j));
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut neighbors = vec![];
    let mut basins = vec![];

    for &low_point in &low_points {
        visited.clear();
        neighbors.push(low_point);
        while let Some((i, j)) = neighbors.pop() {
            for (p, h) in map.neighborhood(i, j) {
                if h < 9 && !visited.contains(&p) {
                    neighbors.push(p);
                    visited.insert(p);
                }
            }
        }
        basins.push(visited.len());
    }

    basins.sort();
    let prod = basins.iter().rev().take(3).fold(1, |p, v| p*v);
    println!("mult: {}", prod);
}
