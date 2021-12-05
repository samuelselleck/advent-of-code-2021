use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

trait CountDuplicates {
    fn count_duplicates(&mut self) -> usize;
}

impl<I, E> CountDuplicates for I
where
    I: Iterator<Item = E>,
    E: Eq + Hash,
{
    fn count_duplicates(&mut self) -> usize {
        let mut counter = HashMap::<E, u32>::new();
        while let Some(key) = self.next() {
            *counter.entry(key).or_default() += 1;
        }
        counter.values().filter(|&&v| v > 1).count()
    }
}

fn main() {
    let vents = fs::read_to_string("vents.txt").expect("file not found.");
    let samples = 1000;
    let duplicates = vents
        .split('\n')
        .map(|l| {
            l.split("->")
                .map(|p| p.split(',').map(|c| c.trim().parse::<i32>().unwrap()))
                .map(|mut o| (o.next().unwrap(), o.next().unwrap()))
        })
        .map(|mut o| (o.next().unwrap(), o.next().unwrap()))
        .map(|(p1, p2)| (p1, (p2.0 - p1.0, p2.1 - p1.1)))
        .map(|(p1, v)| {
            (0..=samples)
                .map(|n| (p1.0 + n * v.0 / samples, p1.1 + n * v.1 / samples))
                .collect::<HashSet<(i32, i32)>>()
        })
        .flatten()
        .count_duplicates();

    println!("duplicates: {}", duplicates)
}
