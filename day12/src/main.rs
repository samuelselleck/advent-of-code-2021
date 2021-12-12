use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    
    let caves = fs::read_to_string("caves.txt").expect("file not found");
    let connections: Vec<_> = caves.lines().map(|l| l.split_once('-').unwrap()).collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (c1, c2) in connections {
        graph.entry(c1).or_insert_with(Vec::new).push(c2);
        graph.entry(c2).or_insert_with(Vec::new).push(c1);
    }

    let n = nbr_paths("start", &graph, &HashSet::from_iter(["start"]), false);
    println!("paths: {}", n);
}

fn nbr_paths(curr: &str, graph: &HashMap<&str, Vec<&str>>, already_visited: &HashSet<&str>, multvisit: bool) -> u32 {
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
                    total += nbr_paths(n, graph, &already_visited, true);
                }
            } else {
                let mut visited = already_visited.clone();
                visited.insert(n);
                total += nbr_paths(n, graph, &visited, multvisit);
            }
        } else {
            total += nbr_paths(n, graph, already_visited, multvisit);
        };
    }
    total
}
