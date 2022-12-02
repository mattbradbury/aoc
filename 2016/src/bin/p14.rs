use itertools::Itertools;
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 14);
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

fn generate_keys(salt: &str, stretch: usize) -> (Vec<(String, usize, char)>, usize) {
    let salt = salt.trim();
    let mut keys = Vec::new();
    let mut count = 0;
    let mut candidates = Vec::new();

    'outer: while keys.len() < 64 {
        // if count == 39 || count == 816{ dbg!(count, &candidates); };
        if count % 1000 == 0 {
            println!("Count: {}", count)
        };
        let index = count;
        count += 1;
        let hash = md5::compute(format!("{}{}", salt, index));
        let mut hash = format!("{:x}", hash);
        for _ in 0..stretch {
            let temp = md5::compute(hash);
            hash = format!("{:x}", temp);
        }
        // if index == 39 || index == 816{ dbg!(count, &candidates, &hash, &keys); };

        let mut hex = hash.chars().tuple_windows();

        let triple = loop {
            match hex.next() {
                Some((a, b, c)) if a == b && b == c => break Some(a),
                None => break None,
                Some(_) => {}
            }
        };
        let triple = if let Some(triple) = triple {
            triple
        } else {
            continue 'outer;
        };
        candidates.retain(|(_, i, _)| i + 1000 > index);

        if let Some((_, b, c)) = hex.nth(1) {
            if b == c && c == triple {
                let (a, b) = candidates
                    .iter()
                    .cloned()
                    .partition::<Vec<_>, _>(|(h, _i, t)| *t == triple);
                candidates.clear();
                keys.extend(a);
                candidates.extend(b);

                // keys.extend(a.iter());

                // continue 'outer
            };
        };
        candidates.push((hash.clone(), index, triple));
    }

    (keys, count - 1)
}

fn part1(input: &str) -> usize {
    let resp = generate_keys(input, 0);
    // dbg!(&resp);
    resp.0[63].1
}

fn part2(input: &str) -> usize {
    let resp = generate_keys(input, 2016);
    dbg!(&resp);

    resp.0[63].1
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "abc";

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 22728)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 22551)
    }
}
