use std::fs;

fn main() {
    //########################## Part 1 ##########################

    let report: Vec<Vec<i32>> = fs::read_to_string("report.txt")
        .expect("file not found.")
        .split('\n')
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&b| 2 * (b == b'1') as i32 - 1)
                .collect()
        })
        .collect();

    let mut counts: Vec<i32> = vec![0; report[0].len()];
    for line in &report {
        for (i, b) in line.iter().enumerate() {
            counts[i] += b;
        }
    }

    let gamma = arr_to_binary(&counts);
    let epsilon = arr_to_binary(&counts.iter().map(|c| -c).collect::<Vec<_>>());

    println!(
        "gamma: {}, epsilon: {}, power consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    //########################## Part 2 ##########################

    let oxygen_gen = arr_to_binary(find_filter_sequence(&report, |c, v| {
        c * v > 0 || (c == 0 && v > 0)
    }));
    let co2_rating = arr_to_binary(find_filter_sequence(&report, |c, v| {
        c * v < 0 || (c == 0 && v < 0)
    }));

    println!(
        "oxygen: {}, co2: {}, life support rating: {}",
        oxygen_gen,
        co2_rating,
        oxygen_gen * co2_rating
    )
}

fn arr_to_binary(arr: &[i32]) -> u32 {
    arr.iter()
        .map(|&b| (b > 0) as u32)
        .rev()
        .enumerate()
        .map(|(i, b)| b << i)
        .sum::<u32>()
}

fn find_filter_sequence<I>(report: &[Vec<i32>], el_filter: I) -> &[i32]
where
    I: Fn(i32, i32) -> bool,
{
    let mut candidates: Vec<_> = (0..report.len()).collect();
    for i in 0..report[0].len() {
        let mut count = 0;
        for &r in &candidates {
            count += report[r][i]
        }
        if candidates.len() > 1 {
            candidates = candidates
                .into_iter()
                .filter(|&r| el_filter(count, report[r][i]))
                .collect();
        } else {
            break;
        }
    }
    &report[candidates[0]]
}
