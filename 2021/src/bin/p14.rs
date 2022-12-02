use aoc_helper::load_input;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, time};

fn main() {
    let input = load_input(2021, 14);
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
fn parse_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let mut lines = input.trim().lines();
    let seq = lines.next().unwrap().trim();
    // println!("'{}'", seq);
    lines.next();
    let map = lines
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().unwrap();
            let b = chars.next().unwrap();
            let c = chars.nth(4).unwrap();
            // println!("{}{} -> {}", a, b, c);
            ((a, b), c)
        })
        .collect::<HashMap<_, _>>();
    // println!("{:#?}", map);
    (seq, map)
}

// fn dig(map: &HashMap<(char,char),char>, acc: &mut HashMap<char,usize>, a: char, b: char, mut count: usize) {
//     if count == 0 { return };
//     let c = map[&(a,b)];
//     *acc.entry(c).or_insert(0) += 1;
//     count -= 1;
//     dig(map, acc, a, c, count);
//     dig(map, acc, c, b, count);
// }

fn grow(base: &str, map: HashMap<(char, char), char>, count: usize) -> usize {
    // let seq = base.trim().chars();
    println!("base.len {}", base.len());
    let mut counts = HashMap::new();

    base.chars()
        .tuple_windows()
        .for_each(|(a, b)| *counts.entry((a, b)).or_insert(0) += 1);

    println!("counts.len {:#?}", counts.len());
    let mut acc = HashMap::new();

    for _ in 0..count {
        for (key, count) in counts.clone().into_iter() {
            *counts.entry(key).or_insert(0) -= count;
            let between = map[&key];
            *counts.entry((key.0, between)).or_insert(0) += count;
            *counts.entry((between, key.1)).or_insert(0) += count;
        }
    }

    for ((a, _), count) in counts {
        *acc.entry(a).or_insert(0) += count;
    }

    *acc.entry(base.chars().last().unwrap()).or_insert(0) += 1;
    acc.values().max().unwrap() - acc.values().min().unwrap()
}

fn part1(input: &str) -> usize {
    let (seq, map) = parse_input(input);
    let res = grow(seq, map, 10);
    res
}

fn part2(input: &str) -> usize {
    let (seq, map) = parse_input(input);
    let res = grow(seq, map, 40);
    res
}

#[cfg(test)]
mod tests {

    use crate::{part1, part2};

    const EXAMPLE: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 1588)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 2188189693529)
    }

    #[test]
    fn test_input_p1() {
        let input = include_str!("../../../input/2021-14.txt");

        assert_eq!(part1(&input), 2657)
    }
}
