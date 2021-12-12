use aoc_helper::load_input;

type Sheet = Vec<Vec<usize>>;

#[derive(Debug)]
struct Card {
    sheet: Sheet,
    marks: Vec<Vec<bool>>,
}

impl Card {
    fn new(sheet: Sheet) -> Card {
        let marks = vec![vec![false; 5]; 5];
        Card { sheet, marks }
    }

    fn mark(&mut self, call: usize) {
        for i in 0..5 {
            for j in 0..5 {
                if self.sheet[i][j] == call {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    continue 'outer;
                }
            }
            return true;
        }

        'outer2: for i in 0..5 {
            for j in 0..5 {
                if !self.marks[j][i] {
                    continue 'outer2;
                }
            }
            return true;
        }

        false
    }

    fn unmarked_sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    sum += self.sheet[i][j];
                }
            }
        }
        sum
    }
}

fn main() {
    let input = load_input(4, 2021);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Card>) {
    let mut input = input.lines();
    let calls = input.next().unwrap();
    let calls = calls
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    // let mut calls = Vec::new();
    let mut cards = Vec::new();
    //  assert_eq!(input.next().unwrap(), ""); // throw away a blank line
    while let Some(_line) = input.next() {
        let sheet = (0..5)
            .map(|_| {
                input
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect()
                // sheet.push(temp);
            })
            .collect();
        println!("{:#?}", sheet);

        cards.push(Card::new(sheet));
        // assert_eq!(input.next().unwrap(), ""); // throw away a blank line
    }

    (calls, cards)
}

fn part1(input: &str) -> usize {
    let (calls, mut cards) = parse_input(input);
    for call in calls {
        for card in cards.iter_mut() {
            card.mark(call);
            if card.bingo() {
                return call * card.unmarked_sum();
            }
        }
    }

    0 //placeholder
}

fn part2(input: &str) -> usize {
    let (calls, mut cards) = parse_input(input);
    for call in calls {
        for card in cards.iter_mut() {
            card.mark(call);
        }
        match cards.len() > 1 {
            true => cards.retain(|card| !card.bingo()),
            false => {
                if cards[0].bingo() {
                    return cards[0].unmarked_sum() * call;
                }
            }
        }
    }

    0 //placeholder
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE), 4512);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE), 1924);
    }
}
