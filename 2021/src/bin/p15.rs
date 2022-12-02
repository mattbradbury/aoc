use aoc_helper::{load_input, point::Point, Grid};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    time,
};

fn main() {
    let input = load_input(2021, 15);
    let grid = parse_input2(&input);
    println!("Part1: {}", bench(|| part1(&input)));
    println!("Part2: {}", bench(|| part2(grid)));
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
fn parse_input(input: &str) -> Grid<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    Grid { grid }
}

fn parse_input2(input: &str) -> Grid<usize> {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    let grid = Grid { grid };

    let (dx, dy) = grid.dimensions();
    let grid2 = vec![vec![0; dx * 5]; dy * 5];

    let (dx, dy) = (dx as isize, dy as isize);
    let mut grid2 = Grid { grid: grid2 };
    let (dx2, dy2) = grid2.dimensions();
    let dx2 = dx2 as isize;
    let dy2 = dy2 as isize;
    for j in 0..dy2 {
        for i in 0..dx2 {
            let val = grid.get(i % dx, j % dy).unwrap();
            let add = i / dx + j / dy;
            let val = val + add as usize;
            let val = if val > 9 { val - 9 } else { val };
            grid2.set(i, j, val)
        }
    }
    grid2
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let (dx, dy) = grid.dimensions();
    let goal = Point {
        x: dx as isize - 1,
        y: dy as isize - 1,
    };
    let mut path = a_star(&grid, Point { x: 0, y: 0 }, goal).unwrap();
    path.reverse();
    println!("Path: {:#?}", path);
    path.into_iter()
        .skip(1)
        .map(|Point { x, y }| grid.get(x, y).unwrap())
        .sum()
}

fn part2(grid: Grid<usize>) -> usize {
    // let grid = parse_input2(input);
    let (dx, dy) = grid.dimensions();
    println!("dx,dy = {},{}", dx, dy);
    let goal = Point {
        x: dx as isize - 1,
        y: dy as isize - 1,
    };
    let path = a_star(&grid, Point { x: 0, y: 0 }, goal).unwrap();
    // path.reverse();
    println!("Path: {:#?}", path);
    // path.into_iter().rev().skip(1).map(|Point {x,y}| grid.get(x, y).unwrap()).sum()
    0
}

fn reconstruct_path(
    came_from: &HashMap<Point<isize>, Point<isize>>,
    mut current: Point<isize>,
) -> Vec<Point<isize>> {
    let mut total_path = vec![current];
    while came_from.keys().contains(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HeapEntry {
    p: Point<isize>,
    f: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

fn _djikstra(grid: &Grid<usize>, start: Point<isize>, _goal: Point<isize>) {
    let (dx, dy) = grid.dimensions();
    let visited = vec![vec![false; dx]; dy];
    let mut unvisited = BinaryHeap::new();
    for x in 0..dx as isize {
        for y in 0..dy as isize {
            if x != 0 && y != 0 {
                unvisited.push(HeapEntry {
                    p: Point { x, y },
                    f: usize::MAX,
                });
            }
        }
    }
    let mut visited = Grid { grid: visited };
    visited.set(start.x, start.y, true);
    let tent_dist = vec![vec![usize::MAX; dx]; dy];
    let mut tent_dist = Grid { grid: tent_dist };
    tent_dist.set(start.x, start.y, 0);
}

fn a_star(
    grid: &Grid<usize>,
    start: Point<isize>,
    goal: Point<isize>,
) -> Option<Vec<Point<isize>>> {
    let (dx, dy) = grid.dimensions();

    let mut open_set = BinaryHeap::new();
    open_set.push(HeapEntry { p: start, f: 0 });
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::with_capacity(dx * dy);
    g_score.insert(start, 0);
    let mut f_score = HashMap::with_capacity(dx * dy);
    f_score.insert(start, start.manhattan(&goal));
    let visited = vec![vec![false; dx]; dy];
    let mut visited = Grid { grid: visited };
    visited.set(start.x, start.y, true);

    while let Some(HeapEntry { p: current, f: _ }) = open_set.pop() {
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        for neigh in grid.get_neighbors_points(current.x, current.y) {
            let tent_g = g_score
                .entry(current)
                .or_insert(usize::MAX)
                .saturating_add(*neigh.0);
            if tent_g < *g_score.entry(neigh.1).or_insert(usize::MAX) {
                came_from.insert(neigh.1, current);
                g_score.insert(neigh.1, tent_g);
                let tent_f = tent_g + current.manhattan(&goal);
                f_score.insert(neigh.1, tent_f);
                if !visited.get(neigh.1.x, neigh.1.y).unwrap() {
                    // if !open_set.iter().any(|he| { he.p == neigh.1 }) {

                    open_set.push(HeapEntry {
                        f: tent_f,
                        p: neigh.1,
                    });
                    visited.set(neigh.1.x, neigh.1.y, true);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const EXAMPLE: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 40)
    }

    // #[test]
    // fn test_example1_p2() {
    //     assert_eq!(part2(EXAMPLE), 315)
    // }
}
