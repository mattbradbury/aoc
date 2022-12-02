use aoc_helper::{load_input, Grid, point::Point, Turtle, turtle::{Dirs, Facing}};
use std::{fmt::Display, time, collections::HashMap, cell::{Cell, RefCell}, default};

fn main() {
    let input = load_input(2017, 19);
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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum CellType {
    Path,
    Letter(char),
    #[default]
    Empty,
}

#[allow(dead_code)]
fn parse_input(input: &str) -> (HashMap<Point<isize>, CellType>, Point<isize>) {
    // let grid = HashMap::new();
    let lines = input.lines();
    let start2 = Cell::new(Point::default());
    let start = &start2;

    let grid = lines.enumerate().flat_map(move |(i,l)| {
        // let i = i.clone();
        // let i = &i;

        l.chars().enumerate().map(move |(j, c):(usize, char) | {
            // let i = i.clone();

            let cell = match c {
                '|' | '-' | '+' => { 
                    if i == 0 { 
                        start.replace(Point {x: (j as isize).clone(), y: (i.clone() as isize).clone() });
                        // dbg!(&start);
                    };
                    CellType::Path 
                },
                letter if letter.is_alphabetic() => { CellType::Letter(letter) },
                _ => { CellType::Empty }
            };
            (Point {y: -(i.clone() as isize), x: j as isize}, cell)
        })
    }).collect::<HashMap<_,_>>();
    (grid, start2.into_inner())
}

fn part1(input: &str) -> String {
    let (mut grid, start) = parse_input(input);
    let mut turtle = Turtle::from(start);
    turtle.facing = Facing::South;
    let mut letters = Vec::new();

    loop {
        turtle.advance(1);
        if let CellType::Letter(l) = grid.entry(turtle.into()).or_insert(CellType::Empty) { letters.push(*l) };
        if *grid.entry(turtle.peek(Dirs::Straight).into()).or_insert(CellType::Empty) != CellType::Empty { continue; }
        if *grid.entry(turtle.peek(Dirs::Left).into()).or_insert(CellType::Empty) != CellType::Empty { 
            turtle.turn(Dirs::Left);
            continue; 
        }
        if *grid.entry(turtle.peek(Dirs::Right).into()).or_insert(CellType::Empty) != CellType::Empty { 
            turtle.turn(Dirs::Right);
            continue; 
        }    
        break;
    }
    letters.into_iter().collect()
}

fn part2(input: &str) -> usize {
    let (mut grid, start) = parse_input(input);
    let mut turtle = Turtle::from(start);
    turtle.facing = Facing::South;
    let mut steps = 1;

    loop {
        turtle.advance(1);
        steps += 1;
        // if let CellType::Letter(l) = grid.entry(turtle.into()).or_insert(CellType::Empty) { letters.push(*l) };
        if *grid.entry(turtle.peek(Dirs::Straight).into()).or_insert(CellType::Empty) != CellType::Empty { continue; }
        if *grid.entry(turtle.peek(Dirs::Left).into()).or_insert(CellType::Empty) != CellType::Empty { 
            turtle.turn(Dirs::Left);
            continue; 
        }
        if *grid.entry(turtle.peek(Dirs::Right).into()).or_insert(CellType::Empty) != CellType::Empty { 
            turtle.turn(Dirs::Right);
            continue; 
        }    
        break;
    }
    steps
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = 

    
r#"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), "ABCDEF")
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 38)
    }
}
