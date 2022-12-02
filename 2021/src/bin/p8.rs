use std::{collections::HashSet, fmt::Display, time};

use aoc_helper::load_input;
use itertools::Itertools;

fn main() {
    let input = load_input(2021, 8);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
}

#[derive(Clone, Debug)]
struct SSD {
    digits: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
    map: [Option<HashSet<char>>; 10],
}

impl SSD {
    fn deduce(&mut self) {
        self.map[1] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 2)
                    .unwrap()
                    .0,
            ),
        );
        self.map[4] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 4)
                    .unwrap()
                    .0,
            ),
        );
        self.map[7] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 3)
                    .unwrap()
                    .0,
            ),
        );
        self.map[8] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 7)
                    .unwrap()
                    .0,
            ),
        );

        self.map[9] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.is_superset(self.map[4].as_ref().unwrap()))
                    .unwrap()
                    .0,
            ),
        );

        self.map[0] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 6 && d.is_superset(self.map[7].as_ref().unwrap()))
                    .unwrap()
                    .0,
            ),
        );

        self.map[6] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 6)
                    .unwrap()
                    .0,
            ),
        );

        self.map[5] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 5 && d.is_subset(self.map[6].as_ref().unwrap()))
                    .unwrap()
                    .0,
            ),
        );

        self.map[3] = Some(
            self.digits.swap_remove(
                self.digits
                    .iter()
                    .find_position(|d| d.len() == 5 && d.is_superset(self.map[7].as_ref().unwrap()))
                    .unwrap()
                    .0,
            ),
        );

        self.map[2] = Some(self.digits.remove(0));
    }

    fn compute(&self) -> usize {
        let mut ret = 0;
        for out in &self.output {
            ret += self
                .map
                .iter()
                .position(|digit| {
                    let d = digit.as_ref().unwrap();
                    d.is_superset(&out) && d.is_subset(&out)
                })
                .unwrap();
            ret *= 10;
        }
        ret / 10
    }
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
fn parse_input(input: &str) -> Vec<SSD> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (digits, output) = line.split_once(" | ").unwrap();
            let digits = digits
                .trim()
                .split_whitespace()
                .map(|digit| digit.chars().collect::<HashSet<_>>())
                .collect_vec();
            let output = output
                .trim()
                .split_whitespace()
                .map(|digit| digit.chars().collect::<HashSet<_>>())
                .collect_vec();
            SSD {
                output,
                digits,
                map: [None, None, None, None, None, None, None, None, None, None],
            }
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let ssds = parse_input(input);
    ssds.iter()
        .map(|ssd| {
            ssd.output
                .iter()
                .map(|o| {
                    if [2, 3, 4, 7].contains(&o.len()) {
                        1_usize
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut ssds = parse_input(input);
    for ssd in ssds.iter_mut() {
        ssd.deduce();
    }
    ssds.iter().map(|ssd| ssd.compute()).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 26)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 61229)
    }
}
