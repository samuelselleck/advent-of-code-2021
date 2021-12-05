use std::collections::HashMap;
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

    let duplicates = vents.split('\n').flat_map(|l| {
        l.split("->")
            .map(|p| p.split(',').map(|c| c.trim().parse::<u32>().unwrap()))
            .map(|mut o| (o.next().unwrap(), o.next().unwrap()))
            .map(|(p1, p2)| p1 + p2) //TODO make this unwrap to all points between
    }).count_duplicates();

    println!("duplicates: {:?}", duplicates)
}
