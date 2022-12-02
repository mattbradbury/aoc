use itertools::Itertools;
use num::Zero;
use pathfinding::prelude::{absdiff, astar};
#[allow(unused_imports)]
use scan_fmt::scan_fmt;
use std::hash::Hash;
use std::{collections::HashMap, fmt::Display, time};

use aoc_helper::load_input;

fn main() {
    // let input = load_input(2016, 0);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1((31, 39, 1358))));
    println!("Part2: {}", bench(|| part2("1358")));
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

fn run<'a>(endx: usize, endy: usize, secret: usize) -> usize {
    let start = (1, 1);
    let success = |(x, y): &(usize, usize)| -> bool { *x == endx && *y == endy };

    let heuristic = |(x, y): &(usize, usize)| -> usize { absdiff(endx, *x) + absdiff(endy, *y) };
    let successors = |(x, y): &(usize, usize)| -> Vec<((usize, usize), usize)> {
        // fn successors((x,y): &(usize,usize)) -> Vec<((usize,usize), usize)> {
        ([(-1, 0), (1, 0), (0, 1), (0, -1)])
            .iter()
            .filter_map(move |(i, j)| {
                let x = *x as isize + i;
                let y = *y as isize + j;
                let x = if x < 0 { return None } else { x as usize };
                let y = if y < 0 { return None } else { y as usize };
                let weird = x * x + x * 3 + x * y * 2 + y + y * y + secret;
                let is_wall = weird.count_ones() % 2 == 0;
                match is_wall {
                    true => Some(((x, y), 1)),
                    false => None,
                }
            })
            .collect_vec()
    };

    let res = astar(&start, successors, heuristic, success);
    res.unwrap().1
}

fn nodes_within_distance<N, C, FN, IN>(start: &N, mut successors: FN, max_cost: C) -> HashMap<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut to_visit = Vec::new();

    to_visit.push((start.clone(), C::zero()));
    let mut visited = HashMap::new();
    while let Some((cur_node, cur_cost)) = to_visit.pop() {
        let neighbors = successors(&cur_node);
        for (neighbor, cost) in neighbors {
            let tentative_cost = cur_cost + cost;

            if tentative_cost > max_cost {
                continue;
            };

            match visited.get(&neighbor) {
                Some(visited_cost) if *visited_cost > tentative_cost => {}
                None => {}
                Some(_) => {
                    continue;
                }
            };
            to_visit.push((neighbor.clone(), tentative_cost));
            visited.insert(neighbor, tentative_cost);
        }
    }

    visited
}

#[allow(dead_code)]
fn parse_input(_input: &str) {}

fn part1(args: (usize, usize, usize)) -> usize {
    run(31, 39, 1358)
}

fn part2(_input: &str) -> usize {
    let successors = |(x, y): &(usize, usize)| -> Vec<((usize, usize), usize)> {
        // fn successors((x,y): &(usize,usize)) -> Vec<((usize,usize), usize)> {
        ([(-1, 0), (1, 0), (0, 1), (0, -1)])
            .iter()
            .filter_map(move |(i, j)| {
                let x = *x as isize + i;
                let y = *y as isize + j;
                let x = if x < 0 { return None } else { x as usize };
                let y = if y < 0 { return None } else { y as usize };
                let weird = x * x + x * 3 + x * y * 2 + y + y * y + 1358;
                let is_wall = weird.count_ones() % 2 == 0;
                match is_wall {
                    true => Some(((x, y), 1)),
                    false => None,
                }
            })
            .collect_vec()
    };
    nodes_within_distance(&(1, 1), successors, 50)
        .keys()
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, run};

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(run(7, 4, 10), 11)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
