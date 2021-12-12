use std::time;

use aoc_helper::load_input;
use aoc_helper::Grid;
use itertools::Itertools;


fn main() {

    bench(|| part1());
    bench(|| part2());
}

fn bench<F>(f: F) where F: FnOnce() {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}

fn parse_input(input: &str) -> Grid<usize> {
    
    let grid = input.trim()
        .lines()
        .map(|line| 
            line.trim()
                .chars()
                .map(|c | c.to_digit( 10).unwrap() as usize)
                .collect_vec()
            )
        .collect_vec();
    Grid { grid }
        
}




fn part1() {
    let input = load_input(2021, 9);
    let input = parse_input(&input);
    let risk = find_risk(&input);
    println!("Part1: {}", risk);
}

fn part2() {
    let input = load_input(2021, 9);
    let mut input = parse_input(&input);
    let bigs = fill_3(&mut input);
    println!("Part2: {}", bigs);
}

fn find_risk(grid: &Grid<usize>) -> usize {
    find_lows(&grid).iter()
        .map(|(x,y)| {
            grid.get(*x, *y).unwrap() + 1
        })
        .sum()
    
}

fn find_lows(grid: &Grid<usize>) -> Vec<(isize, isize)> {
    let dims = grid.dimensions();
    (0..dims.0 as isize).map(|x| {
        (0..dims.1 as isize).map(|y| {
            let val = grid.get(x, y).unwrap();
            let smaller = grid.get_neighbors(x, y).iter().fold(true, |acc, n| {
                acc && *n > val
            });
            if smaller { Some((x,y)) } else { None }
        }).collect_vec() 
    }).flatten()
    .filter_map(|xy| xy)
    .collect_vec()
}

fn fill_basin(grid: &mut Grid<usize>, x: isize ,y: isize) -> usize {
    match grid.get(x, y) {

        Some(val) if *val == 9  => { return 0 }
        None => { return 0 },
        Some(_) => {
            grid.set(x, y, 9);
            fill_basin(grid, x + 1, y) +
            fill_basin(grid, x - 1, y) +
            fill_basin(grid, x, y + 1) +
            fill_basin(grid, x, y - 1) + 1
        }
    }
}

fn fill_3(grid: &mut Grid<usize>) -> usize {
    let mut lows = find_lows(grid);
    let mut fills = lows.iter_mut()
        .map(|xy| {
            fill_basin(grid, xy.0, xy.1)
        }).collect_vec();
    fills.sort();
    fills.reverse();
    return fills[0] * fills[1] * fills[2];
}

#[cfg(test)]
mod tests {
    use crate::{find_risk, parse_input, fill_3};

    const EXAMPLE: &str = 
r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_example1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_risk(&input),15);
    }

    #[test]
    fn test_example1_p2() {
        let mut input = parse_input(EXAMPLE);
        assert_eq!(fill_3(&mut input),1134);
    }
}
