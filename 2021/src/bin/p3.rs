use aoc_helper::load_input;

fn main() {
    let input = load_input(3, 2021);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut result = vec![0; 12];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                result[i] += 1
            }
            if c == '0' {
                result[i] -= 1
            }
        }
    }

    let mut eps = 0;
    let mut gam = 0;
    // println!("{:?}", result);
    for (i, v) in result.iter().rev().enumerate() {
        match *v > 0 {
            true => gam += 2_usize.pow(i as u32),
            false => eps += 2_usize.pow(i as u32),
        }
    }
    // println!("result: {}", eps * gam)
    eps * gam
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);

    find_rating(Gen::Oxy, &input) * find_rating(Gen::CO2, &input)
}

#[derive(Clone, Copy)]
enum Gen {
    Oxy,
    CO2,
}

#[allow(clippy::ptr_arg)]
fn find_rating(gen: Gen, input: &Vec<Vec<char>>) -> usize {
    let mut data = input.to_owned();
    for pos in 0..data[0].len() {
        if data.len() == 1 {
            break;
        };
        let count = get_pos_count(&data, pos);
        println!("i, c: {}, {}", pos, count);
        let discriminant = match gen {
            Gen::Oxy => count >= 0,
            Gen::CO2 => count < 0,
        };

        #[allow(clippy::match_like_matches_macro)]
        data.retain(|entry| match (gen, discriminant, entry[pos]) {
            (Gen::Oxy, true, '1') => true,
            (Gen::Oxy, false, '0') => true,
            (Gen::CO2, true, '1') => true,
            (Gen::CO2, false, '0') => true,

            _ => false,
        })
    }
    usize::from_str_radix(&data[0].iter().collect::<String>(), 2).unwrap()
}

fn get_pos_count(arr: &[Vec<char>], pos: usize) -> isize {
    arr.iter().fold(0, |mut a, v| {
        a += match v[pos] {
            '1' => 1,
            '0' => -1,
            _ => 0,
        };
        a
    })
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {

    use crate::{find_rating, parse_input, Gen};

    const EXAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn test_oxy() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_rating(Gen::Oxy, &input), 23);
    }

    #[test]
    fn test_co2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_rating(Gen::CO2, &input), 10);
    }
}
