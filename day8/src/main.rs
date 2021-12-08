use std::collections::HashSet;
use std::fs;

fn main() {
    let str_file = fs::read_to_string("digits.txt").expect("file not found.");
    let lines: Vec<&str> = str_file.split('\n').collect();

    let mut sum = 0;
    for l in lines {
        let mut itr = l.split('|').map(|a| {
            a.split_whitespace()
                .map(|d| {
                    let mut chars: Vec<char> = d.chars().collect();
                    chars.sort_by(|a, b| b.cmp(a));
                    String::from_iter(chars)
                })
                .collect()
        });

        let (examples, problems): (Vec<String>, Vec<String>) =
            (itr.next().unwrap(), itr.next().unwrap());
        let all: HashSet<_> = examples.iter().chain(problems.iter()).collect();

        let ext_dig = |f: Box<dyn Fn(&str) -> bool>| -> &str {
            &all.iter().filter(|s| f(s)).next().unwrap().as_ref()
        };

        let one = ext_dig(Box::new(|s| s.len() == 2));
        let four = ext_dig(Box::new(|s| s.len() == 4));
        let seven = ext_dig(Box::new(|s| s.len() == 3));
        let eight = ext_dig(Box::new(|s| s.len() == 7));
        let three = ext_dig(Box::new(|s| s.len() == 5 && dig_df(s, one) == 0));
        let nine = ext_dig(Box::new(|s| s.len() == 6 && dig_df(s, four) == 0));
        let six = ext_dig(Box::new(|s| s.len() == 6 && dig_df(s, seven) != 0));
        let zero = ext_dig(Box::new(|s| s.len() == 6 && s != six && s != nine));
        let two = ext_dig(Box::new(|s| s.len() == 5 && dig_df(s, four) > 1));
        let five = ext_dig(Box::new(|s| s.len() == 5 && s != two && s != three));

        let nums = [zero, one, two, three, four, five, six, seven, eight, nine];
        sum += problems
            .iter()
            .map(|c| nums.iter().position(|d| d == c).unwrap())
            .rev()
            .enumerate()
            .map(|(i, v)| v as u32 * 10u32.pow(i as u32))
            .sum::<u32>();
    }

    println!("sum: {}", sum);
}

fn dig_df(digit1: &str, digit2: &str) -> usize {
    digit2.chars().filter(|&c| !digit1.contains(c)).count()
}
