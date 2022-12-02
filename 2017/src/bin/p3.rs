use aoc_helper::{load_input, Turtle};
use std::{fmt::Display, time, collections::HashMap};
use aoc_helper::turtle::Dirs;

type Grid = HashMap<(isize, isize), usize>;

fn main() {
    let input = load_input(2017, 3);
    let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(input)));
    println!("Part2: {}", bench(|| part2(input)));
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
fn parse_input(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn part1(input: usize) -> usize {
    let root = (input as f64).sqrt().floor() as usize;
    let root = if root % 2 == 0 { root - 1} else { root };
    let side = root + 2;
    let left = input - (root * root);
    let layer = side / 2 ;
    if left == 0 { return root - 1 };
    let pos = left % (root + 1);
    let offset = ((side / 2) as isize - pos as isize).abs() as usize;
    println!("root: {root} side: {side} left: {left} pos: {pos} offset: {offset} layer: {layer}");
    layer + offset
    
}

fn part2(input:usize) -> usize {
    let mut turtle = Turtle::default();
    let mut grid = HashMap::<(isize,isize), usize>::new();
    grid.insert((&turtle).into(), 1);
    use crate::Dirs::*;
    loop {
        let (x,y) = turtle.peek(Left);
        // println!("Peek: {},{}", x, y);
        if !(&grid).contains_key(&(&turtle).peek(Left)) {
            turtle.turn(Left);
        }
        turtle.advance(1);
        let sum = sum_neighbors(&grid, (&turtle).into());
        if sum > input { break sum };
        grid.insert((&turtle).into(), sum);
        // println!("Sum: {sum} Turtle: {turtle}");
    }
}

fn sum_neighbors(grid: &Grid, (x,y): (isize, isize)) -> usize {
    let mut sum = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; };
            sum += grid.get(&(x+i, y+j)).unwrap_or(&0);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(12), 3)
    }

    #[test]
    fn test_example2_p2() {
        assert_eq!(part1(21), 4)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(800), 806)
    }


}
