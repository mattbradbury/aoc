use itertools::Itertools;
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::{fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    let input = load_input(2016, 16);
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

fn fold_data(input: &str) -> String {
    let b = input
        .chars()
        .rev()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => panic!(),
        })
        .collect::<String>();

    format!("{}0{}", input, b)
}

fn checksum(input: &str) -> String {
    input
        .chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect()
}

fn fill_disk(init: &str, disk_size: usize) -> String {
    let mut folded = init.to_string();
    while folded.len() < disk_size {
        folded = fold_data(folded.as_str())
    }
    let mut crc = checksum(&folded[0..disk_size]);
    while crc.len() % 2 == 0 {
        crc = checksum(crc.as_str())
    }
    crc
}

fn part1(input: &str) -> String {
    fill_disk(input.trim(), 272)
}

fn part2(input: &str) -> String {
    fill_disk(input.trim(), 35651584)
}

#[cfg(test)]
mod tests {
    use crate::{fill_disk, part1, part2};

    const EXAMPLE: &str = "10000";

    #[test]
    fn test_example1_p1() {
        assert_eq!(fill_disk(EXAMPLE, 20), "01100")
    }

    // #[test]
    // fn test_example1_p2() {
    //     assert_eq!(part2(EXAMPLE), 0)
    // }
}
