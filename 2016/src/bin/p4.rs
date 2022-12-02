use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{collections::HashMap, fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 4);
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
fn parse_input(input: &str) -> Vec<(HashMap<char, usize>, usize, String)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut dashes = line.split('-').rev();
            let (id, crc) = scan_fmt!(dashes.next().unwrap(), "{d}[{}]", usize, String).unwrap();
            let name = dashes
                .map(|s| s.chars())
                .flatten()
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0_usize) += 1;
                    acc
                });
            (name, id, crc)
        })
        .collect_vec()
}

fn parse_input2(input: &str) -> Vec<(String, HashMap<char, usize>, usize, String)> {
    input
        .trim()
        .lines()
        .map(|line| {
            // let mut dashes = line.split('-')
            // .rev();
            let (name, id, crc) =
                scan_fmt!(line, "{[-a-z]}{d}[{}]", String, usize, String).unwrap();
            let chars = name
                .chars()
                .filter(|s| *s != '-')
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0_usize) += 1;
                    acc
                });
            (name, chars, id, crc)
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse_input(&input);
    input
        .into_iter()
        .filter_map(|(mut set, id, crc)| {
            let mut crc = crc.chars();
            while let Some(c) = crc.next() {
                let max = *set.values().max().unwrap();
                if set.entry(c).or_insert(0) == &max {
                    set.remove(&c);
                } else {
                    return None;
                }
            }

            Some(id)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let input = parse_input2(&input);
    let _filtered = input
        .into_iter()
        .filter_map(|(name, mut set, id, crc)| {
            let mut crc = crc.chars();
            while let Some(c) = crc.next() {
                let max = *set.values().max().unwrap();
                if set.entry(c).or_insert(0) == &max {
                    set.remove(&c);
                } else {
                    return None;
                }
            }

            Some((name, id))
        })
        .map(|(name, id)| {
            let name = name
                .chars()
                .map(|c| match c {
                    '-' => ' ',
                    _ => ((((c as usize - 97) + id) % 26) as u8 + 97) as char,
                })
                .collect::<String>();
            (name, id)
        })
        .for_each(|v| println!("{}{}", v.0, v.1));
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 0)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
