use std::collections::HashMap;

use std::fs;

struct BingoBoard {
    els: HashMap<u8, (usize, usize)>,
    col_counts: [u8; 5],
    row_counts: [u8; 5],
}

impl BingoBoard {
    fn new(els: HashMap<u8, (usize, usize)>) -> BingoBoard {
        BingoBoard {
            els,
            col_counts: [0; 5],
            row_counts: [0; 5],
        }
    }

    fn mark(&mut self, guess: u8) -> Option<u32> {
        self.els.remove(&guess).and_then(|(r, c)| {
            self.col_counts[c] += 1;
            self.row_counts[r] += 1;
            if self.col_counts[c] == 5 || self.row_counts[r] == 5 {
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
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = lines
        .filter(|&l| l != "")
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
                            .map(|(j, c)| (c.parse().unwrap(), (i, j)))
                    })
                    .flatten()
                    .collect(),
            )
        })
        .collect();

    //Part 1

    for &guess in &guesses {
        
    }

    //Part 2
    let mut guessed: HashSet<u8> = HashSet::new();
    let mut prev_lost = &boards[2];
    for &guess in &guesses {
        guessed.insert(guess);
        let lost = boards.iter().filter(|b| !b.has_won(&guessed)).next();
        match lost {
            Some(board) => prev_lost = board,
            None => {
                println!("last winning score: {}", prev_lost.score(&guessed, guess));
                break;
            }
        }
    }
}
