use std::{fmt::Display, time};

use aoc_helper::{
    load_input,
    point::{BoundedPoint, Point},
    Grid,
};
use itertools::Itertools;

fn main() {
    let input = load_input(2016, 2);
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
fn parse_input(_input: &str) {}

fn part1(input: &str) -> String {
    let mut pos = BoundedPoint::new(1..4, 1..4);

    input
        .trim()
        .lines()
        .map(|line| {
            line.chars().for_each(|c| match c {
                'R' => pos += Point { x: 1, y: 0 },
                'L' => pos += Point { x: -1, y: 0 },
                'U' => pos += Point { x: 0, y: -1 },
                'D' => pos += Point { x: 0, y: 1 },
                _ => {
                    panic!("Unexpected direction {}", c)
                }
            });
            let cur = pos.get();
            format!("{}", (cur.y - 1) * 3 + cur.x)
        })
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let pad = r#"  1  
 234 
56789
 ABC 
  D  "#;
    let mut pos = BoundedPoint::new(0..5, 0..5);
    pos.set(Point { x: 0, y: 2 });

    let grid = pad.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let grid = Grid { grid };

    input
        .trim()
        .lines()
        .map(|line| {
            line.chars().for_each(|c| {
                let prev = pos.clone();
                match c {
                    'R' => pos += Point { x: 1, y: 0 },
                    'L' => pos += Point { x: -1, y: 0 },
                    'U' => pos += Point { x: 0, y: -1 },
                    'D' => pos += Point { x: 0, y: 1 },
                    _ => {
                        panic!("Unexpected direction {}", c)
                    }
                }

                if grid[&pos] == ' ' {
                    pos = prev.clone()
                };
                // println!("{}", pos);
            });
            // let cur = pos.get();
            format!("{}", grid[&pos])
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"ULL
RRDDD
LURDL
UUUUD
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), "1985".to_owned())
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), "5DB3".to_owned())
    }
}
