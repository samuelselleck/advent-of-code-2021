use std::fs;

const CHUNK_STARTS: [char; 4] = ['<', '(', '[', '{'];
const CHUNK_ENDS: [char; 4] = ['>', ')', ']', '}'];

fn delims_match(start: char, end: char) -> bool {
    CHUNK_STARTS.iter().position(|&c| c == start) == CHUNK_ENDS.iter().position(|&c| c == end)
}

fn main() {
    let nav_system: Vec<_> = fs::read_to_string("nav.txt")
        .expect("file not found.")
        .lines()
        .map(|l| interpret(l.chars()))
        .collect();

    let mut err_sum = 0;
    let mut comp_scores = vec![];

    for c in nav_system {
        match c {
            Err(v) => {
                err_sum += match v {
                    '}' => 1197,
                    ')' => 3,
                    ']' => 57,
                    '>' => 25137,
                    _ => 0,
                }
            }
            Ok(v) => comp_scores.push(v.iter().rev().fold(0u128, |s, c| {
                5 * s
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            })),
        }
    }
    comp_scores.sort();
    let middle_score = comp_scores[comp_scores.len() / 2];

    println!("error sum: {}", err_sum);
    println!("comp sum: {:?}", middle_score);
}

fn interpret(chars: impl Iterator<Item = char>) -> Result<Vec<char>, char> {
    let mut token_stack = vec![];
    for c in chars {
        if CHUNK_STARTS.contains(&c) {
            token_stack.push(c)
        } else if CHUNK_ENDS.contains(&c) {
            if delims_match(*token_stack.last().unwrap(), c) {
                token_stack.pop();
            } else {
                return Err(c);
            }
        }
    }
    Ok(token_stack)
}
