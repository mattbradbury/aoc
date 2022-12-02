use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{collections::VecDeque, fmt::Display, time};

use aoc_helper::load_input;

enum Command {
    Rect { w: usize, h: usize },
    RotateRow { row: usize, dist: usize },
    RotateCol { col: usize, dist: usize },
}

fn main() {
    let input = load_input(2016, 8);
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
fn parse_input(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (command, rest) = line.split_once(' ').unwrap();
            match command {
                "rect" => {
                    let (w, h) = scan_fmt!(rest.trim(), "{}x{}", usize, usize).unwrap();
                    Command::Rect { w, h }
                }
                "rotate" => {
                    println!("rest={}", rest);
                    let (_, axis, rc, dist) =
                        scan_fmt!(rest.trim(), "{} {}={} by {}", String, char, usize, usize)
                            .unwrap();
                    match axis {
                        'x' => Command::RotateCol { col: rc, dist },
                        'y' => Command::RotateRow { row: rc, dist },
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let commands = parse_input(input);
    let row = VecDeque::from(vec![false; 50]);
    let mut display = vec![row.clone(); 6];

    for command in commands {
        match command {
            Command::Rect { w, h } => {
                for j in 0..h {
                    for i in 0..w {
                        display[j][i] = true;
                    }
                }
            }
            Command::RotateRow { row, dist } => {
                for _ in 0..dist {
                    let temp = display[row].pop_back().unwrap();
                    display[row].push_front(temp);
                }
            }
            Command::RotateCol { col, dist } => {
                for _ in 0..dist {
                    let t = display[5][col];
                    for y in (1..6).rev() {
                        display[y][col] = display[y - 1][col];
                    }
                    display[0][col] = t;
                }
            }
        }
    }
    for v in &display {
        let s: String = v.iter().map(|lit| if *lit { '#' } else { ' ' }).collect();
        println!("{}", s);
    }
    display
        .iter()
        .map(|vd| vd.iter().filter(|&&lit| lit))
        .flatten()
        .count()
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 6)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
