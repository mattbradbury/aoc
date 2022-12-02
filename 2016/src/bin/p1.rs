use std::{collections::HashSet, fmt::Display, time};

use aoc_helper::load_input;
use itertools::FoldWhile::*;
use itertools::Itertools;

fn main() {
    let input = load_input(2016, 1);
    let input = parse_input(&input);
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

enum Dir {
    Left,
    Right,
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<(Dir, i32)> {
    input
        .trim()
        .split(", ")
        .map(|val| {
            let mut chars = val.chars();
            let dir = match chars.next().unwrap() {
                'L' => Dir::Left,
                'R' => Dir::Right,
                bad @ _ => panic!("Unexpected char {}", bad),
            };
            let chars = chars.collect::<String>().parse().unwrap();
            (dir, chars)
        })
        .collect_vec()
}

fn part1(input: &Vec<(Dir, i32)>) -> i32 {
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]; //Right = + left = -
    let mut facing = 0;

    let finalpos: (i32, i32) = input.iter().fold((0, 0), |pos, (dir, dist)| {
        facing = match dir {
            Dir::Left => (facing + 3) % 4,
            Dir::Right => (facing + 1) % 4,
        };
        (pos.0 + dirs[facing].0 * dist, pos.1 + dirs[facing].1 * dist)
    });
    finalpos.0.abs() + finalpos.1.abs()
}

fn part2(input: &Vec<(Dir, i32)>) -> i32 {
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]; //Right = + left = -
    let mut facing = 0;

    let (finalpos, _) = input
        .iter()
        .fold_while(
            ((0_i32, 0_i32), HashSet::new()),
            |(mut pos, mut set), (dir, dist)| {
                facing = match dir {
                    Dir::Left => (facing + 3) % 4,
                    Dir::Right => (facing + 1) % 4,
                };
                for _ in 0..*dist {
                    pos = (pos.0 + dirs[facing].0, pos.1 + dirs[facing].1);
                    println!("{:?}", pos);
                    if set.contains(&pos) {
                        return Done((pos, set));
                    } else {
                        set.insert(pos)
                    };
                }

                Continue((pos, set))
            },
        )
        .into_inner();
    finalpos.0.abs() + finalpos.1.abs()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const EXAMPLE: &str = r#"R5, L5, R5, R3"#;

    #[test]
    fn test_example1_p1() {
        let input = parse_input(EXAMPLE);

        assert_eq!(part1(&input), 12)
    }

    #[test]
    fn test_example1_p2() {
        let example = "R8, R4, R4, R8";
        let input = parse_input(example);

        assert_eq!(part2(&input), 4)
    }
}
