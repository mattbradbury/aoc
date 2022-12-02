use aoc_helper::load_input;
use itertools::*;
use std::{fmt::Display, time};

// #############
// #...........#
// ###A#C#C#D###
//   #B#D#A#B#
//   #########

fn main() {
    let start: State = [N, N, N, N, N, N, N, A, B, C, D, C, A, D, B];

    // let input = load_input(2021, 0);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(start)));
    // println!("Part2: {}", bench(|| part2(&input)));
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Occupier {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    N = 4,
}

impl Display for Occupier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            N => "E",
        };
        write!(f, "{}", out)
    }
}
use pathfinding::prelude::astar;
use Occupier::*;
// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
type State = [Occupier; 15];
//     A:[u8; 2],
//     B:[u8; 2],
//     C:[u8; 2],
//     D:[u8; 2],
// }
const ROOMS: [[usize; 2]; 4] = [[8, 7], [10, 9], [12, 11], [14, 13]];
const SUCCESS: State = [N, N, N, N, N, N, N, A, A, B, B, C, C, D, D];
// #############
// #01.2.3.4.56#
// ###7#9#1#3###
//   #8#0#2#4#
//   #########

fn find_next_states<'a>(current: &State) -> Vec<(State, usize)> {
    let costs: [Vec<(usize, usize)>; 15] = [
        vec![(1, 1)],
        vec![(0, 1), (7, 2), (2, 2)],
        vec![(1, 2), (7, 2), (9, 2), (3, 2)],
        vec![(2, 2), (9, 2), (11, 2), (4, 2)],
        vec![(3, 2), (11, 2), (13, 2), (5, 2)],
        vec![(4, 2), (13, 2), (6, 1)],
        vec![(5, 1)],
        vec![(1, 2), (2, 2), (8, 1)],
        vec![(7, 1)],
        vec![(2, 2), (3, 2), (10, 1)],
        vec![(9, 1)],
        vec![(3, 2), (4, 2), (12, 1)],
        vec![(11, 1)],
        vec![(4, 2), (5, 2), (14, 1)],
        vec![(13, 1)],
    ];

    let mut next_states = Vec::new();
    for (pos, occupant) in current.iter().enumerate() {
        if *occupant == N {
            continue;
        };
        let mut targets = Vec::new();
        if current[ROOMS[*occupant as usize][1]] == current[ROOMS[*occupant as usize][0]]
            && current[ROOMS[*occupant as usize][1]] == *occupant
        {
            continue;
        };
        if ROOMS[*occupant as usize][0] == pos {
            continue;
        };
        if pos > 6 {
            targets.append(&mut vec![0, 1, 2, 3, 4, 5, 6])
        };
        if current[ROOMS[*occupant as usize][0]] == *occupant {
            targets.push(ROOMS[*occupant as usize][1])
        };
        targets.push(ROOMS[*occupant as usize][0]);
        let multiplier = match occupant {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
            N => panic!(),
        };

        let next = |node: &usize| -> Vec<(usize, usize)> {
            costs[*node]
                .iter()
                .cloned()
                .filter_map(|nexthop| {
                    if current[nexthop.0] == N {
                        Some(nexthop)
                    } else {
                        None
                    }
                })
                .collect_vec()
        };

        let h = |_node: &usize| 0;

        let results = targets.iter().filter_map(|target| {
            let s = |node: &usize| node == target;
            match astar(&pos, next, h, s) {
                Some((_path, cost)) => {
                    let mut newstate = current.clone();
                    newstate[pos] = N;
                    newstate[*target] = *occupant;
                    Some((newstate, cost * multiplier))
                }
                None => None,
            }
        });
        next_states.extend(results);
    }
    next_states
}

fn print_state(state: &State) {
    print!("[");
    for o in state {
        print!("{:?},", o);
    }
    println!("]");
}

// #############
// #01.2.3.4.56#
// ###7#9#1#3###
//   #8#0#2#4#
//   #########

#[test]
fn next_states() {
    const EXAMPLE: State = [N, N, N, N, N, N, N, B, A, C, D, B, C, D, A];
    const EXAMPLE2: State = [N, C, N, N, N, N, N, A, A, B, B, N, D, D, C];
    const EXAMPLE3: State = [N, N, N, A, D, N, N, N, A, B, B, C, C, N, D];
    // [N,N,N,N,N,N,N,B,A,C,D,B,C,D,A,]
    // [N,N,N,N,D,N,N,B,A,C,D,B,C,N,A,]
    // [N,N,N,N,D,A,N,B,A,C,D,B,C,N,N,]
    // [N,N,B,N,D,A,N,B,A,C,D,N,C,N,N,]
    // [N,N,B,N,D,A,N,B,A,N,D,C,C,N,N,]
    // [N,N,B,N,N,A,N,B,A,N,D,C,C,N,D,]
    // [N,N,B,N,D,A,N,B,A,N,N,C,C,N,D,]
    // [N,N,N,N,D,A,N,B,A,N,B,C,C,N,D,]
    // [N,N,N,N,D,A,N,N,A,B,B,C,C,N,D,] 2006
    // [N,N,N,N,D,N,N,A,A,B,B,C,C,N,D,] 2000
    // [N,N,N,N,N,N,N,A,A,B,B,C,C,D,D,]
    let states = find_next_states(&EXAMPLE3);
    print!("      ");
    print_state(&EXAMPLE3);
    for (s, c) in &states {
        print!("{:5} ", c);
        print_state(s);
    }
    println!();
    // dbg!(states);

    let res = path_find(EXAMPLE3);

    dbg!(res);
}

fn path_find(start: State) -> usize {
    let result = astar(
        &start,
        |current| find_next_states(current),
        |_cur| 1,
        |cur| *cur == SUCCESS,
    )
    .unwrap();
    // dbg!(&result);
    for state in result.0 {
        print_state(&state);
    }
    result.1
}

#[allow(dead_code)]
fn parse_input(_input: &str) {}

fn part1(start: State) -> usize {
    path_find(start)
}

fn part2(start: State) -> usize {
    0
}

// #############
// #01.2.3.4.56#
// ###7#9#1#3###
//   #8#0#2#4#
//   #########

// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
#[cfg(test)]
mod tests {
    use crate::{part1, part2, Occupier::*, State};

    const EXAMPLE: State = [N, N, N, N, N, N, N, B, A, C, D, B, C, D, A];

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 12521)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
