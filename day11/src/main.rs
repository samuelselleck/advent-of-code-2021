use std::fs;
use std::collections::HashSet;

struct SquidSquad {
    energies: Vec<Vec<u8>>,
    dims: (i32, i32),
}

impl SquidSquad {
    fn new(energies: Vec<Vec<u8>>) -> SquidSquad {
        SquidSquad {
            dims: (energies.len() as i32, energies[0].len() as i32),
            energies
        }
    }

    fn get_height(&mut self, i: i32, j: i32) -> Option<&mut u8> {
        self.energies.get_mut(i as usize)
            .and_then(|v: &mut Vec<_>| v.get_mut(j as usize))
    }

    fn neighborhood(i: i32, j: i32) -> impl Iterator<Item = (i32, i32)> {
        [(1, 0), (-1, 0), (0, 1), (0, -1), (-1, -1), (1, -1), (-1, 1), (1, 1)]
            .iter()
            .map(move |(di, dj)| (i + di, j + dj))
    }

    fn flash(&mut self) -> bool {

        let mut flashed = HashSet::new();

        let (w, h) = self.dims;
        for i in 0..w {
            for j in 0..h {
                *self.get_height(i, j).unwrap() += 1;
            }
        }

        loop {
            let mut to_flash = vec![];
            for i in 0..w {
                for j in 0..h {
                    if *self.get_height(i, j).unwrap() >= 10 && !flashed.contains(&(i, j)) {
                        to_flash.push((i, j));
                    }
                }
            }

            if to_flash.len() == 0 {
                break;
            }

            for (i, j) in &to_flash {
                flashed.insert((*i, *j));
                for (di, dj) in SquidSquad::neighborhood(*i, *j) {
                    if let Some(v) = self.get_height(di, dj) {
                        *v += 1;
                    }
                }
            }

        }

        for (i, j) in &flashed {
            *self.get_height(*i, *j).unwrap() = 0;
        }

        if flashed.len() as i32 == self.dims.0*self.dims.1 {
            return true;
        }

        false
    }
}

fn main() {
    let mut map = SquidSquad::new(
        fs::read_to_string("energies_t.txt")
            .expect("file not found.")
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    );

    for i in 0..1000 {
        if map.flash() {
            println!("{}", i + 1);
            break;
        }
    }
}

