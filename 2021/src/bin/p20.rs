use aoc_helper::{load_input, point::Point};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time,
};

fn main() {
    let input = load_input(2021, 20);
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

type Image = HashMap<Point<isize>, bool>;

#[allow(dead_code)]
fn parse_input(input: &str) -> (Image, Vec<bool>) {
    let (algo, image) = input.trim().split_once("\n\n").unwrap();

    let algo = algo.chars().map(|c| c == '#').collect_vec();

    let image = image
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices().map(move |(x, c)| {
                let b = c == '#';
                let x = x as isize;
                let y = y as isize;
                (Point { x, y }, b)
            })
        })
        .flatten()
        .collect::<HashMap<_, _>>();
    (image, algo)
}

fn run_algo(image: &mut Image, algo: &Vec<bool>, pass: usize) {
    let orig = image.clone();
    let _to_visit = orig.keys().cloned().collect_vec();
    let to_visit = image
        .keys()
        .map(|k| k.neigh9())
        .flatten()
        .collect::<HashSet<_>>();
    // let to_visit = to_visit.iter().map(|k| k.neigh9()).flatten().collect::<HashSet<_>>();
    // let to_visit = to_visit.iter().map(|k| k.neigh9()).flatten().collect::<HashSet<_>>();
    // let to_visit = to_visit.iter().map(|k| k.neigh9()).flatten().collect::<HashSet<_>>();
    // let to_visit = to_visit.iter().map(|k| k.neigh9()).flatten().collect::<HashSet<_>>();

    to_visit.into_iter().for_each(|point| {
        let lookup = point
            .neigh9()
            .iter()
            .map(|p| {
                match orig.get(p) {
                    Some(b) => *b,
                    None => {
                        // image.insert(*p, pass % 2 == 1);
                        pass % 2 == 1
                    }
                }
                // *orig.entry(*p).or_insert(pass % 2 == 1)
            })
            .fold(0, |mut acc, bit| {
                acc = acc << 1;
                acc | (bit as usize)
            });
        image.insert(point, algo[lookup]);
    })
}

fn printmap(image: &Image) {
    let (xmin, xmax) = image.keys().minmax_by_key(|p| p.x).into_option().unwrap();
    let (ymin, ymax) = image.keys().minmax_by_key(|p| p.y).into_option().unwrap();
    for y in ymin.y..=ymax.y {
        for x in xmin.x..=xmax.x {
            print!(
                "{}",
                if *image.get(&Point { x, y }).unwrap() {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!();
    println!();
}

fn part1(input: &str) -> usize {
    let (mut image, algo) = parse_input(input);

    for pass in 0..50 {
        printmap(&image);
        run_algo(&mut image, &algo, pass)
    }
    printmap(&image);

    image.values().filter(|p| **p).count()
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 35)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
