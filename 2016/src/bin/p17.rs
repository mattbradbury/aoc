use aoc_helper::{MaxPQEntry, MinPQEntry};
use itertools::Itertools;
use num::Zero;
use pathfinding::{directed::astar, prelude::astar};
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::{fmt::Display, panic, time};

use aoc_helper::{
    load_input,
    point::{BoundedPoint, Point},
};

fn main() {
    let input = load_input(2016, 17);
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct State {
    loc: Point<u8>,
    steps: String,
}

#[test]
fn test() {
    let secret = "ihgpwlah";
    let state = State {
        loc: Point { x: 3, y: 2 },
        steps: "DRRR".to_owned(),
    };
    let input = format!("{}{}", secret, state.steps.trim());
    let hash = format!("{:x}", md5::compute(dbg!(&input)));
    dbg!(hash);
    let res = next_hops("ihgpwlah", &state);
    dbg!(res);
}

fn next_hops(secret: &str, state: &State) -> Vec<(State, usize)> {
    let hash = format!(
        "{:x}",
        md5::compute(format!("{}{}", secret, state.steps.trim()))
    );

    let dirs = ['U', 'D', 'L', 'R'].into_iter();
    hash[0..4]
        .chars()
        .zip(dirs)
        .filter_map(|(h, dir)| {
            if "0123456789a".contains(h) {
                return None;
            };
            let point = match dir {
                'U' if state.loc.y != 0 => Point {
                    x: state.loc.x,
                    y: state.loc.y - 1,
                },
                'D' if state.loc.y != 3 => Point {
                    x: state.loc.x,
                    y: state.loc.y + 1,
                },
                'L' if state.loc.x != 0 => Point {
                    x: state.loc.x - 1,
                    y: state.loc.y,
                },
                'R' if state.loc.x != 3 => Point {
                    x: state.loc.x + 1,
                    y: state.loc.y,
                },
                _ => return None,
            };
            let mut steps = format!("{}{}", &state.steps.trim(), dir);
            // if steps.len() > 4 {
            //     steps = (&steps[1..5]).to_owned()
            // };

            let newstate = State { loc: point, steps };
            Some((newstate, 1))
        })
        .collect_vec()
}

fn find_path(secret: &str) -> String {
    let secret = secret.trim();
    let start = State {
        loc: Point::default(),
        steps: "".to_owned(),
    };
    let heuristic = |_: &State| 1;
    let success = |state: &State| state.loc == Point { x: 3, y: 3 };

    let successors = move |state: &State| -> Vec<(State, usize)> { next_hops(secret, state) };

    let result = astar(&start, successors, heuristic, success).unwrap();
    result
        .0
        .into_iter()
        .skip(1)
        .filter_map(|state| {
            // dbg!(&state.steps);
            state.steps.chars().last()
        })
        .collect()
}

fn find_longest_path(secret: &str) -> usize {
    let secret = secret.trim();
    let start = State {
        loc: Point::default(),
        steps: "".to_owned(),
    };
    // let heuristic = |_: &State| 1;
    let success = |state: &State| state.loc == Point { x: 3, y: 3 };

    let successors = |state: &State| -> Vec<(State, usize)> { next_hops(secret, state) };

    let result = dfs_longest(&start, successors, success);
    result.unwrap().1
}

fn part1(input: &str) -> String {
    find_path(input)
}

fn part2(input: &str) -> usize {
    find_longest_path(input)
}

fn dfs_longest<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> Option<(N, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_visit = BinaryHeap::new();
    let start = MaxPQEntry::<N, C> {
        data: start.clone(),
        priority: Zero::zero(),
    };
    to_visit.push(start);

    let mut max_found: Option<(N, C)> = None;

    while let Some(current) = to_visit.pop() {
        let next_hops = successors(&current.data);
        for (next_hop, cost) in next_hops {
            let found = success(&next_hop);
            if found {
                match max_found {
                    Some(max) if max.1 < cost + current.priority => {
                        max_found = Some((next_hop, cost + current.priority))
                    }
                    None => { max_found = Some((next_hop, cost + current.priority)) }
                    _ => {}
                };
                continue;
            }
            let entry = MaxPQEntry::new(cost + current.priority, next_hop);
            to_visit.push(entry);
        }
    }

    max_found
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE1: &str = "ihgpwlah";
    const EXAMPLE2: &str = "kglvqrro";
    const EXAMPLE3: &str = "ulqzkmiv";

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE1), "DDRRRD");
        assert_eq!(part1(EXAMPLE2), "DDUDRLRRUDRD");
        assert_eq!(part1(EXAMPLE3), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE1), 370);
    }
}
