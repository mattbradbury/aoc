use itertools::{FoldWhile, Itertools};
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 7);
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
fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let output = input
        .trim()
        .lines()
        .map(|line| {
            line.split(|c| ['[', ']'].contains(&c)).enumerate().fold(
                (Vec::new(), Vec::new()),
                |(mut noth, mut hype), (i, s)| {
                    // let s = s.to_owned();
                    if i % 2 == 1 {
                        hype.push(s)
                    } else {
                        noth.push(s)
                    }
                    (noth, hype)
                },
            )
        })
        .collect_vec();

    output
}

fn has_abba(input: &str) -> bool {
    input
        .chars()
        .tuple_windows()
        .fold_while(false, |_, (a, b, c, d)| {
            if a == b {
                return FoldWhile::Continue(false);
            };
            if a == d && b == c {
                return FoldWhile::Done(true);
            };
            FoldWhile::Continue(false)
        })
        .into_inner()
}

fn get_aba(input: &Vec<&str>) -> Vec<(char, char)> {
    // (a, b) of aba
    input
        .iter()
        .map(|s| {
            s.chars()
                .tuple_windows()
                .filter_map(
                    |(a, b, c)| {
                        if a == c && a != b {
                            Some((a, b))
                        } else {
                            None
                        }
                    },
                )
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn has_bab(input: &Vec<&str>, a: char, b: char) -> bool {
    input.iter().any(|line| {
        line.chars()
            .tuple_windows()
            .any(|(x, y, z)| x == b && y == a && z == b)
    })
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    // println!("{:#?}", input);
    input
        .iter()
        .filter(|(norm, hype)| {
            if hype.iter().any(|s| has_abba(s)) {
                return false;
            };
            if norm.iter().any(|s| has_abba(s)) {
                return true;
            };
            false
        })
        .count()
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .filter(|(norm, hype)| {
            // println!("{:?}", norm);
            let abas = get_aba(norm);
            // println!("{:#?}", abas);
            abas.into_iter().any(|(a, b)| has_bab(hype, a, b))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
"#;

    const EXAMPLE2: &str = r#"aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 2)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE2), 3)
    }
}
