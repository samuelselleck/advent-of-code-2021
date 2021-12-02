use std::fs;

fn main() {
    let instructions = fs::read_to_string("instructions.txt").expect("file not found.");

    let commands = instructions
        .split('\n')
        .map(|s| s.split_whitespace())
        .map(|mut itr| {
            (
                itr.next().expect("no command."),
                itr.next().expect("no command argument.")
                    .parse::<i32>()
                    .expect("argument not an integer."),
            )
        });

    let [mut depth, mut horiz, mut aim] = [0; 3];
    for (cmd, arg) in commands {
        match cmd {
            "down" => aim += arg,
            "up" => aim -= arg,
            "forward" => {
                horiz += arg;
                depth += aim * arg;
            }
            cmd => panic!("unknown command: {}", cmd),
        }
    }

    println!(
        "horizontal: {}, depth: {}, mult: {}",
        horiz,
        depth,
        horiz * depth
    );
}
