use aoc_helper::{load_input, hexturtle::{HexDir, HexTurtle}};
use itertools::Itertools;
use std::{fmt::Display, time, cmp::max};

fn main() {
    let input = load_input(2017, 11);
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
fn parse_input(input: &str) -> Vec<HexDir> {
    input.trim().split(",").map(|p| p.try_into().unwrap()).collect_vec()
}

fn part1(input: &str) -> isize {
    let dirs = parse_input(input);
    let mut cursor = HexTurtle::default();
    for dir in dirs {
        cursor.go(dir);
    }
    cursor.distance_to(&HexTurtle::default())
    
}

fn part2(input: &str) -> isize {
    let dirs = parse_input(input);
    let mut cursor = HexTurtle::default();
    let origin = HexTurtle::default();
    let mut far = 0;
    for dir in dirs {
        cursor.go(dir);
        far = max(far, cursor.distance_to(&origin));
    }
    far
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1("se,sw,se,sw,sw"), 3)
    }

    #[test]
    fn test_example2_p1() {
        assert_eq!(part1("ne,ne,sw,sw"), 0)
    }
}
