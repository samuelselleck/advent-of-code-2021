use std::collections::HashMap;

use std::fs;

struct BingoBoard {
    els: HashMap<u8, (usize, usize)>,
    col_counts: [u8; 5],
    row_counts: [u8; 5],
    won: bool,
}

impl BingoBoard {
    fn new(els: HashMap<u8, (usize, usize)>) -> BingoBoard {
        BingoBoard {
            els,
            col_counts: [0; 5],
            row_counts: [0; 5],
            won: false,
        }
    }

    fn mark(&mut self, guess: u8) -> Option<u32> {
        self.els.remove(&guess).and_then(|(r, c)| {
            self.col_counts[c] += 1;
            self.row_counts[r] += 1;
            if !self.won && (self.col_counts[c] == 5 || self.row_counts[r] == 5) {
                self.won = true;
                Some(self.score(guess))
            } else {
                None
            }
        })
    }

    fn score(&self, winning_num: u8) -> u32 {
        self.els.keys().map(|&e| e as u32).sum::<u32>() * (winning_num as u32)
    }
}

fn main() {
    let bingo_info = fs::read_to_string("bingo.txt").expect("file not found.");
    let mut lines = bingo_info.split('\n');
    let guesses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.trim().parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = lines
        .filter(|&l| l.trim() != "")
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|board_str| {
            BingoBoard::new(
                board_str
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        l.split_whitespace()
                            .enumerate()
                            .map(move |(j, c)| (c.trim().parse().unwrap(), (i, j)))
                    })
                    .flatten()
                    .collect(),
            )
        })
        .collect();

    let events: Vec<u32> = guesses
        .iter()
        .map(|&g| {
            boards
                .iter_mut()
                .filter_map(|b| b.mark(g))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    println!(
        "first winner score: {}, last winner score: {}",
        events.first().unwrap(),
        events.last().unwrap()
    )
}
