use std::{collections::VecDeque, time};

use aoc_helper::load_input;

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}
fn main() {
    bench(part1);
    bench(part1);
    bench(part2);
}

fn part1() {
    let input = load_input(6, 2021);
    let input = input.trim();
    let fish = parse_input(input);
    let res = count_fish(fish, 80);
    println!("Part1: {}", res);
}

fn part2() {
    let input = load_input(6, 2021);
    let input = input.trim();
    let fish = parse_input(input);
    let res = count_fish(fish, 256);
    println!("Part2: {}", res);
}

fn parse_input(input: &str) -> VecDeque<usize> {
    let mut result = VecDeque::new();
    (0..9).for_each(|_| result.push_back(0));
    input
        .split(',')
        .for_each(|v| result[v.parse::<usize>().unwrap()] += 1);
    result
}

fn count_fish(mut fish: VecDeque<usize>, days: usize) -> usize {
    for _day in 0..days {
        let spawns = fish.pop_front().unwrap();
        fish[6] += spawns;
        fish.push_back(spawns);
    }
    fish.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{count_fish, parse_input};

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn example1() {
        let fish = parse_input(EXAMPLE);
        assert_eq!(count_fish(fish, 18), 26);
    }

    #[test]
    fn example2() {
        let fish = parse_input(EXAMPLE);
        assert_eq!(count_fish(fish, 80), 5934);
    }

    #[test]
    fn example3() {
        let fish = parse_input(EXAMPLE);
        assert_eq!(count_fish(fish, 256), 26984457539);
    }
}
