use std::{time, fmt::Display};

use aoc_helper::{load_input, Grid};
use itertools::Itertools;

fn main() {
    let input = load_input(2021, 11);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(&input)));
}

fn bench<F, T>(f: F) -> T where F: FnOnce() -> T , T:Display {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Energy(u8),
    Flashed,
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Grid<Cell> {
    Grid { grid : input.trim().lines().map(|line| {
        line.trim().chars().map(|c| Cell::Energy(c.to_digit(10).unwrap() as u8)).collect_vec()
    }).collect_vec()}

}

fn step(grid: &mut Grid<Cell>) -> usize {
    let (dimx,dimy) = grid.dimensions();
    let mut count = 0;
    let mut prev = 0;
    const NEIGHS:[(isize, isize); 8] =     
        [
        (-1,-1), (0,-1), (1,-1),
        (-1,0),          (1,0),
        (-1,1),  (0,1),  (1,1)];

    let mut firstpass = true;
    let mut lastpass = false;

    loop {
        for j in 0..dimy as isize {
            for i in 0..dimx as isize {
                let mut val = grid.get_mut(i, j).unwrap();
                match (firstpass, lastpass, &mut val) {
                    (true, false, Cell::Energy(e)) => { *e += 1 },
                    (false, true, Cell::Flashed) => { 
                        *val = Cell::Energy(0);
                        continue;
                    },
                    (_,_,_) => {}
                }
                if let Cell::Energy(energy) = val {                
                    if *energy > 9 { 
                        // println!("{} {}", i, j);
                        // replace(val, Cell::Flashed);
                        *val = Cell::Flashed;
                        count += 1;
                        drop(val);
                        for (dx, dy) in NEIGHS {
                            if let Some(cell) = grid.get_mut(i + dx, j + dy) { 
                                match cell {
                                    Cell::Energy(n) => { *n += 1 },
                                    _ => {},
                                }
                            };
                        }
                    };
                }
            } 
        }
        if lastpass { return count }

        if count == prev { lastpass = true }
        prev = count;
        firstpass = false;
    }
}

fn check_flashed(grid: &Grid<Cell>) -> bool {
    let (dimx, dimy) = grid.dimensions();
    for j in 0..dimy as isize {
        for i in 0..dimx as isize {
            match grid.get(i, j).unwrap() {
                Cell::Energy(val) if *val == 0 => {},
                _ => { return false }
            }
        }
    }
    true
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    (0..100).map(|_| step(&mut grid)).sum()
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut count = 0;
    loop {

        step(&mut grid);
        count += 1;
        if check_flashed(&grid) { break count }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 1656)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 195)
    }
}
