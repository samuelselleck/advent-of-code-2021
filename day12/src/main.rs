use std::fs;
use std::collections::{HashMap, HashSet};
use std::time::Instant;


fn main() {
    
    let caves = fs::read_to_string("caves.txt").expect("file not found");
    let connections: Vec<_> = caves.lines().map(|l| l.split_once('-').unwrap()).collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (c1, c2) in connections {
        graph.entry(c1).or_insert_with(Vec::new).push(c2);
        graph.entry(c2).or_insert_with(Vec::new).push(c1);
    }

    let timer = Instant::now();
    let n = nbr_paths("start", &graph, &mut HashSet::from_iter(["start"]), false);
    let dur = timer.elapsed();
    println!("paths: {} in {:?}", n, dur);
}

fn nbr_paths<'a>(curr: &str, graph: &HashMap<&'a str, Vec<&'a str>>, already_visited: &mut HashSet<&'a str>, multvisit: bool) -> u32 {
    if curr == "end" {
        return 1;
    }
    let mut next = graph.get(curr).unwrap().clone();
    next.retain(|c| !already_visited.contains(c) || !multvisit);
    let mut total = 0;
    for n in next {
        if n == n.to_lowercase() {
            if already_visited.contains(n) {
                if n != "start" {
                    total += nbr_paths(n, graph, already_visited, true);
                }
            } else {
                already_visited.insert(n);
                total += nbr_paths(n, graph, already_visited, multvisit);
                already_visited.remove(n);
            }
        } else {
            total += nbr_paths(n, graph, already_visited, multvisit);
        };
    }
    total
}
