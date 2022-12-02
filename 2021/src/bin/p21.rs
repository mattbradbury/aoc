use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    time,
};

fn main() {
    // let input = load_input(2021, 21);
    // let input = parse_input(&input);
    println!("Part1: {}", bench(|| part1(5, 8)));
    println!("Part2: {}", bench(|| part2(5, 8)));
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

#[derive(Debug)]
pub struct DeterministicDie {
    last: usize,
    roll_count: usize,
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.roll_count += 1;
        self.last += 1;
        if self.last > 100 {
            self.last = 1
        };
        Some(self.last)
    }
}

impl DeterministicDie {
    pub fn new() -> Self {
        Self {
            last: 100,
            roll_count: 0,
        }
    }

    pub fn roll_count(&self) -> usize {
        self.roll_count
    }
}

fn play(p1_pos: usize, p2_pos: usize) -> usize {
    let mut pos = [p1_pos - 1, p2_pos - 1]; // board is labeled 1-10, indexed 0-9
    let mut score = [0, 0];
    let mut cur_player = 0; // players 1 and 2 are indexed as 0 and 1
    let die = &mut DeterministicDie::new();

    while score[0] < 1000 && score[1] < 1000 {
        let roll: usize = die.take(3).sum();
        pos[cur_player] += roll;
        pos[cur_player] %= 10;
        score[cur_player] += pos[cur_player] + 1;
        cur_player = (cur_player + 1) % 2;
    }

    let loser = min(score[0], score[1]);
    loser * die.roll_count()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct GameState {
    pos: [u8; 2],
    score: [u8; 2],
    cur_player: u8,
    // rolls: usize,
    winner: Option<u8>,
}

#[test]
fn size() {
    println!("{}", std::mem::size_of::<GameState>())
}

fn turn(mut gs: GameState, roll: u8) -> GameState {
    gs.pos[gs.cur_player as usize] += roll;
    gs.pos[gs.cur_player as usize] %= 10;
    gs.score[gs.cur_player as usize] += gs.pos[gs.cur_player as usize] + 1;
    gs.cur_player = (gs.cur_player + 1) % 2;
    // dbg!(score);
    if gs.score[0] >= 21 || gs.score[1] >= 21 {
        gs.winner = Some(gs.cur_player)
    }
    gs
}

fn qplay(p1_pos: u8, p2_pos: u8) -> usize {
    const DIST: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    let pos = [p1_pos - 1, p2_pos - 1]; // board is labeled 1-10, indexed 0-9
    let score = [0, 0];
    let cur_player = 0; // players 1 and 2 are indexed as 0 and 1
    let gs = GameState {
        pos,
        score,
        cur_player,
        winner: None,
    };
    let rolls = [3, 4, 5, 6, 7, 8, 9];
    let mut playing = HashMap::new();
    let mut finished = Vec::new();

    playing.insert(gs, 0);
    let mut first_turn = true;

    while playing.len() > 0 {
        let mut after_turn = HashMap::new();
        playing.drain().for_each(|(gs, prev_rolls)| {
            for roll in rolls {
                let roll_count = if !first_turn {
                    prev_rolls * DIST[roll as usize]
                } else {
                    prev_rolls + DIST[roll as usize]
                };
                let gs = turn(gs, roll);
                *after_turn.entry(gs).or_insert(0) += roll_count;
            }
        });
        finished.extend(after_turn.iter().filter_map(|(gs, rolls)| {
            if gs.winner.is_some() {
                Some((*gs, *rolls))
            } else {
                None
            }
        }));
        // playing.clear();
        after_turn.retain(|gs, _| gs.winner.is_none());
        playing = after_turn;
        first_turn = false;
    }

    finished.len();

    let res = finished.into_iter().fold([0, 0], |mut acc, (gs, rolls)| {
        let winner = gs.winner.unwrap() as usize;
        acc[winner] += rolls;
        acc
    });
    max(res[0], res[1])
}

fn part1(player1: usize, player2: usize) -> usize {
    play(player1, player2)
}

fn part2(player1: u8, player2: u8) -> usize {
    qplay(player1, player2)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(4, 8), 739785)
    }

    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(4, 8), 444356092776315)
    }
}
