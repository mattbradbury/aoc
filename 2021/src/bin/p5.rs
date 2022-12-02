use aoc_helper::{load_input, point::Point, timer};

fn main() {
    timer(part1);
    timer(part2);
}

fn part1() {
    let input = load_input(2021, 5);
    let lines = parse_input(&input);

    println!("Woohoo Part1: {}", find_vents(lines, Mode::NoDiag))
}

fn part2() {
    let input = load_input(2021, 5);
    let lines = parse_input(&input);

    println!("Part2: {}", find_vents(lines, Mode::Diag))
}

#[derive(PartialEq, Eq)]
enum Mode {
    Diag,
    NoDiag,
}

fn find_vents(lines: Vec<(Point<usize>, Point<usize>)>, mode: Mode) -> usize {
    let mut array = vec![vec![0; 1000]; 1000];
    // println!("{:?}", lines);
    for (a, b) in lines {
        if !(a.x == b.x || a.y == b.y) && mode == Mode::NoDiag {
            continue;
        };
        let (mut i, mut j) = (a.x, a.y);
        loop {
            array[i][j] += 1;
            // println!("{}, {} = {}", i, j, array[i][j]);
            if b.x == i && b.y == j {
                break;
            }

            match a.y.cmp(&b.y) {
                std::cmp::Ordering::Less => j += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => j -= 1,
            }

            match a.x.cmp(&b.x) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => i -= 1,
            }
        } // end outer
    }

    array.iter().flatten().filter(|v| **v > 1).count()
}

fn parse_input(input: &str) -> Vec<(Point<usize>, Point<usize>)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            (a.try_into().unwrap(), b.try_into().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{find_vents, parse_input};

    const EXAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn test_example1() {
        let lines = parse_input(EXAMPLE);

        assert_eq!(find_vents(lines, crate::Mode::NoDiag), 5);
    }

    #[test]
    fn test_example2() {
        let lines = parse_input(EXAMPLE);

        assert_eq!(find_vents(lines, crate::Mode::Diag), 12);
    }
}
