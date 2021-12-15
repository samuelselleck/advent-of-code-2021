use std::collections::HashMap;
use std::fs;

fn main() {
    let poly = fs::read_to_string("poly.txt").expect("file not found");
    let mut itr = poly.lines();

    let template: &[u8] = itr.next().unwrap().as_bytes();
    let mut pairs: HashMap<_,u64> = template.windows(2).map(|c| (c, 1)).collect();
    itr.next();
    let rules: HashMap<&[u8], ([u8; 2], [u8; 2])> = itr
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(p, v)| (p.as_bytes(), v.as_bytes()))
        .map(|(p, v)| (p, ([p[0], v[0]], [v[0], p[1]])))
        .collect();

    let mut next: HashMap<&[u8], _> = HashMap::new();
    for _ in 0..40 {
        for (k, v) in pairs {
            let (p1, p2) = rules.get(k).unwrap();
            *next.entry(p1).or_insert(0) += v;
            *next.entry(p2).or_insert(0) += v;
        }
        pairs = next;
        next = HashMap::new();
    }

    let mut elements = HashMap::new();
    for (k, v) in pairs {
        *elements.entry(k[0]).or_insert(0) += v;
        *elements.entry(k[1]).or_insert(0) += v;
    }
    *elements.get_mut(template.first().unwrap()).unwrap() += 1;
    *elements.get_mut(template.last().unwrap()).unwrap() += 1;

    let counts: Vec<_> = elements.iter().map(|(k, v)| (*k as char, v / 2)).collect();
    let min = counts.iter().min_by_key(|(_, v)| v).unwrap().1;
    let max = counts.iter().max_by_key(|(_, v)| v).unwrap().1;
    println!("{:?}", max - min);
}
