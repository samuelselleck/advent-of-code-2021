use std::fs;
use std::collections::HashSet;

fn main() {
    let origami = fs::read_to_string("extreme.txt").expect("file not found");

    let mut itr = origami.lines();

    let mut dots: Vec<[i32; 2]> = vec![];
    while let Some((x, y)) = itr.next().and_then(|l| l.split_once(',')) {
        dots.push([x.parse().unwrap(), y.parse().unwrap()]);
    }

    let divisions: Vec<(usize, i32)> = itr
        .map(|l| l.split_once('=').unwrap())
        .map(|(s1, v)| {
            (
                match s1.chars().last().unwrap() {
                    'x' => 0,
                    'y' => 1,
                    _ => panic!(),
                },
                v.parse().unwrap(),
            )
        })
        .collect();

    for (i, c) in divisions {
        for p in &mut dots {
            p[i] = c - (c - p[i]).abs()
        }
    }

    let points = HashSet::<[i32; 2]>::from_iter(dots);
    let max_x = points.iter().max_by_key(|p| p[0]).unwrap()[0] + 1;
    let max_y = points.iter().max_by_key(|p| p[1]).unwrap()[1] + 1;

    let mut screen = vec![vec![' ';max_x as usize]; max_y as usize];

    for [x, y] in points {
        screen[y as usize][x as usize] = '#';
    }
    
    for l in screen {
        println!("{}", l.iter().collect::<String>())
    }
}
