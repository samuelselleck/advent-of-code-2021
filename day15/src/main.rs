use pathfinding::prelude::dijkstra;
use std::fs;

fn main() {
    let input = fs::read_to_string("risks.txt").expect("file not found");

    let risk_map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let (w, h) = (risk_map[0].len() as i32, risk_map.len() as i32);

    let s = 5;
    let get_risk =
        |(x, y)| (risk_map[(y % h) as usize][(x % w) as usize] + y / h + x / w - 1) % 9 + 1;

    let res: Option<(_, i32)> = dijkstra(
        &(0, 0),
        |&(x, y)| {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&(x, y)| x >= 0 && x < w * s && y >= 0 && y < h * s)
                .map(|p| (p, get_risk(p)))
        },
        |p| *p == (w * s - 1, h * s - 1),
    );

    println!("{:?}", res.unwrap().1)
}
