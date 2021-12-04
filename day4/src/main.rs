use std::collections::HashSet;
use std::fs;

struct BingoBoard {
    els: Vec<Vec<u8>>,
}

impl BingoBoard {
    fn has_won(&self, guessed: &HashSet<u8>) -> bool {
        let n = self.els.len();
        let mut candidate_rows = vec![true; n];
        let mut candidate_cols = vec![true; n];
        for i in 0..n {
            for j in 0..n {
                candidate_rows[i] = candidate_rows[i] && guessed.contains(&self.els[i][j]);
                candidate_cols[i] = candidate_cols[i] && guessed.contains(&self.els[j][i]);
            }
        }
        return candidate_rows.iter().any(|&b| b) || candidate_cols.iter().any(|&b| b);
    }

    fn score(&self, guessed: &HashSet<u8>, winning_num: u8) -> u32 {
        self.els
            .iter()
            .flatten()
            .filter(|e| !guessed.contains(e))
            .map(|&e| e as u32)
            .sum::<u32>()
            * (winning_num as u32)
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
        .map(|board_str| BingoBoard {
            els: board_str
                .iter()
                .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
                .collect(),
        })
        .collect();

    let mut guessed: HashSet<u8> = HashSet::new();
    for &guess in &guesses {
        guessed.insert(guess);
        let won = boards.iter().filter(|b| b.has_won(&guessed)).next();
        if let Some(board) = won {
            println!("fist winning score: {}", board.score(&guessed, guess));
            break;
        }
    }

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
