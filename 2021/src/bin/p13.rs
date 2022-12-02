use std::{collections::HashSet, fmt::Display, time};

use aoc_helper::load_input;
use itertools::Itertools;

fn main() {
    let input = load_input(2021, 13);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
}

fn bench<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Display,
{
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[allow(dead_code)]
fn parse_input(input: &str) -> (Vec<(isize, isize)>, Vec<(&str, isize)>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let p1 = p1
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(',')
                .map(|v| v.parse::<isize>().unwrap())
                .tuple_windows()
                .next()
                .unwrap();
            (x, y)
        })
        .collect_vec();

    let p2 = p2
        .trim()
        .lines()
        .map(|line| {
            let (axis, val) = line
                .trim_start_matches("fold along ")
                .split_once('=')
                .unwrap();
            let val = val.parse::<isize>().unwrap();
            (axis, val)
        })
        .collect_vec();
    (p1, p2)
}

fn part1(input: &str) -> usize {
    let (points, folds) = parse_input(&input);
    let mut grid = HashSet::new();
    let (axis, val) = folds[0];
    println!("{} = {}", axis, val);
    for point in points {
        let point = match axis {
            "x" => {
                if point.0 > val {
                    (val - (point.0 - val), point.1)
                } else {
                    point
                }
            }
            "y" => {
                panic!();
                // if point.1 > val { (point.0, val - (point.1 - val)) } else { point }
            }
            _ => {
                panic!()
            }
        };
        grid.insert(point);
    }

    grid.len()
}

fn part2(input: &str) -> usize {
    println!();
    let (mut points, folds) = parse_input(&input);
    let mut grid = HashSet::new();
    let mut oldgrid;

    // println!("{} = {}", axis, val);
    for (axis, val) in folds {
        grid = HashSet::new();

        for point in points {
            let point = match axis {
                "x" => {
                    if point.0 > val {
                        (val - (point.0 - val), point.1)
                    } else {
                        point
                    }
                }
                "y" => {
                    if point.1 > val {
                        (point.0, val - (point.1 - val))
                    } else {
                        point
                    }
                }
                _ => {
                    panic!()
                }
            };
            grid.insert(point);
        }
        oldgrid = grid.clone();
        points = oldgrid.into_iter().collect_vec();
    }
    let (mx, _) = *grid.iter().max_by_key(|(x, _)| x).unwrap();
    let (_, my) = *grid.iter().max_by_key(|(_, y)| y).unwrap();
    for j in 0..=my {
        for i in 0..=mx {
            print!("{}", if grid.contains(&(i, j)) { '#' } else { ' ' });
        }
        println!();
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 17)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
