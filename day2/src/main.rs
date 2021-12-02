use std::fs;

fn main() {
    let instructions = fs::read_to_string("instructions.txt").expect("file not found.");

    let commands: Vec<(&str, i32)> = instructions
        .split('\n')
        .map(|s| s.split_whitespace())
        .map(|mut itr| 
            (
                itr.next().expect("no command."),
                itr.next().expect("no command argument.")
                    .parse()
                    .expect("argument not an integer."),
            )
        ).collect();

    let [mut depth, mut horiz, mut aim] = [0; 3];
    for c in commands {
        match c {
            ("down", x) => aim += x,
            ("up", x) => aim -= x,
            ("forward", x) => {
                horiz += x;
                depth += aim * x;
            }
            t => panic!("unknown command: {}", t.0),
        }
    }

    println!(
        "horizontal: {}, depth: {}, mult: {}",
        horiz,
        depth,
        horiz * depth
    );
}
