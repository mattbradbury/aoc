use aoc_helper::load_input;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::{fmt::Display, ops::AddAssign, panic, str::FromStr, time};

fn main() {
    let input = load_input(2021, 18);
    // let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_owned();
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

#[derive(Debug, Clone)]
struct Tree {
    arena: Vec<Pair>,
    root: usize,
}

impl Tree {
    fn new() -> Self {
        Self {
            root: 0,
            arena: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.arena.len()
    }

    fn push(&mut self, pair: Pair) {
        self.arena.push(pair)
    }

    fn reduce(&mut self) {
        loop {
            // println!("{}", self);
            match self.find_exploder(self.root, 0) {
                Some(idx) => {
                    // println!("exploder: {}", self.arena[idx].as_string(&self.arena));
                    self.explode(idx);
                    continue;
                }
                None => {}
            };

            match self.find_splitter(self.root) {
                Some((idx, dir)) => {
                    // println!("splitter: {}", self.arena[idx].as_string(&self.arena));
                    self.split(idx, dir);
                    continue;
                }
                None => break,
            }
        }
    }

    fn explode(&mut self, idx: usize) {
        let pair = &self.arena[idx].clone();
        let (lval, rval) = match (&pair.l, &pair.r) {
            (Part::Val(l), Part::Val(r)) => (l, r),
            (_, _) => panic!(),
        };
        let _cur_idx = idx;

        self.explode_left(idx, *lval);

        ///////// Explode right now

        self.explode_right(idx, *rval);

        //////  Replace pair with val(0)

        let parent = pair.parent.unwrap();
        // println!("{:?}, {:?}", self.arena[parent].l, self.arena[parent].r);
        match (&self.arena[parent].l, &self.arena[parent].r) {
            (Part::Child(i), _) if *i == idx => self.arena[parent].l = Part::Val(0),
            (_, Part::Child(i)) if *i == idx => self.arena[parent].r = Part::Val(0),
            (_, _) => panic!(),
        }
    }

    fn explode_left(&mut self, idx: usize, value: usize) {
        let mut cur_idx = idx;

        cur_idx = loop {
            let prev_idx = cur_idx;
            cur_idx = match self.arena[cur_idx].parent {
                Some(i) => i,
                None => return,
            };
            match self.arena[cur_idx].l {
                Part::Child(child_idx) if child_idx == prev_idx => continue,
                Part::Child(child_idx) => break child_idx,
                Part::Val(v) => {
                    self.arena[cur_idx].l = Part::Val(v + value);
                    return;
                }
            }
        };
        loop {
            match self.arena[cur_idx].r {
                Part::Child(child_idx) => cur_idx = child_idx,
                Part::Val(v) => {
                    self.arena[cur_idx].r = Part::Val(v + value);
                    return;
                }
            }
        }
    }

    fn explode_right(&mut self, idx: usize, value: usize) {
        let mut cur_idx = idx;

        cur_idx = loop {
            let prev_idx = cur_idx;
            cur_idx = match self.arena[cur_idx].parent {
                Some(i) => i,
                None => return,
            };
            match self.arena[cur_idx].r {
                Part::Child(child_idx) if child_idx == prev_idx => continue,
                Part::Child(child_idx) => break child_idx,
                Part::Val(v) => {
                    self.arena[cur_idx].r = Part::Val(v + value);
                    return;
                }
            }
        };
        loop {
            match self.arena[cur_idx].l {
                Part::Child(child_idx) => cur_idx = child_idx,
                Part::Val(v) => {
                    self.arena[cur_idx].l = Part::Val(v + value);
                    return;
                }
            }
        }
    }

    fn split(&mut self, idx: usize, dir: char) {
        let part = match dir {
            'l' => self.arena[idx].l.inner(),
            'r' => self.arena[idx].r.inner(),
            _ => panic!(),
        };
        let l = Part::Val(part / 2);
        let r = Part::Val(part - l.inner());
        let new_idx = self.size();
        let pair = Pair {
            idx: new_idx,
            l,
            r,
            parent: Some(idx),
        };
        self.arena.push(pair);
        match dir {
            'l' => self.arena[idx].l = Part::Child(new_idx),
            'r' => self.arena[idx].r = Part::Child(new_idx),
            _ => panic!(),
        }
    }

    fn find_splitter(&mut self, pair_id: usize) -> Option<(usize, char)> {
        let pair = &self.arena[pair_id].clone();
        // match (depth, &pair.l, &pair.r) {
        //     (4, Part::Val(_), Part::Val(_)) => return Some(pair_id),
        //     (_,_,_) => {}
        // }
        let left = match pair.l {
            Part::Child(child_id) => self.find_splitter(child_id),
            Part::Val(v) if v > 9 => return Some((pair_id, 'l')),
            Part::Val(_) => None,
        };
        if left.is_some() {
            return left;
        };
        match pair.r {
            Part::Child(child_id) => self.find_splitter(child_id),
            Part::Val(v) if v > 9 => Some((pair_id, 'r')),
            Part::Val(_) => None,
        }
    }

    fn find_exploder(&mut self, pair_id: usize, depth: usize) -> Option<usize> {
        let pair = &self.arena[pair_id].clone();
        match (depth, &pair.l, &pair.r) {
            (4, Part::Val(_), Part::Val(_)) => return Some(pair_id),
            (_, _, _) => {}
        }
        let left = match pair.l {
            Part::Child(child_id) => self.find_exploder(child_id, depth + 1),
            Part::Val(_) => None,
        };
        if left.is_some() {
            return left;
        };
        match pair.r {
            Part::Child(child_id) => self.find_exploder(child_id, depth + 1),
            Part::Val(_) => None,
        }
    }

    fn magnitude(&self) -> usize {
        // dbg!(self);
        self._magnitude(self.root)
    }

    fn _magnitude(&self, node: usize) -> usize {
        // dbg!(node);
        let l = match self.arena[node].l {
            Part::Child(node) => self._magnitude(node),
            Part::Val(v) => v,
        };
        let r = match self.arena[node].r {
            Part::Child(node) => self._magnitude(node),
            Part::Val(v) => v,
        };
        (l * 3) + (r * 2)
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = Tree::new();
        // let mut arena = Vec::new();
        let root = Pair::build_from_str(&mut tree, None, s).unwrap();
        // dbg!(root);
        tree.root = root;
        // tree.reduce();
        Ok(tree)
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.arena[self.root].as_string(&self.arena);
        write!(f, "{}", s)
    }
}

// impl AddAssign<&mut Tree> for Tree {
//     fn add_assign(&mut self,  rhs: &mut Self) {
//         (*self).add_assign(rhs)
//     }
// }

impl AddAssign for Tree {
    fn add_assign(&mut self, mut rhs: Self) {
        let lhs_size = self.size();
        let _rhs_size = rhs.size();
        rhs.arena.iter_mut().for_each(|p| {
            for part in [&mut p.r, &mut p.l] {
                match part {
                    Part::Child(v) => *v += lhs_size,
                    Part::Val(_) => {}
                }
            }
            match &mut p.parent {
                Some(v) => *v += lhs_size,
                None => {}
            }
            p.idx += lhs_size;
        });
        self.arena.append(&mut rhs.arena);

        let lpart = Part::Child(self.root);
        let rpart = Part::Child(rhs.root + lhs_size);
        let new_size = self.arena.len();
        self.arena
            .iter_mut()
            .for_each(|pair| match &mut pair.parent {
                Some(_) => {}
                None => pair.parent = Some(new_size),
            });
        let new_root = Pair {
            idx: new_size,
            l: lpart,
            r: rpart,
            parent: None,
        };
        self.arena.push(new_root);
        self.root = new_size;
        // dbg!(&self);
        self.reduce();
    }
}

#[derive(Debug, Clone)]
enum Part {
    Child(usize), // index to Pair arena
    Val(usize),
}

impl Part {
    fn inner(&self) -> usize {
        match self {
            Part::Child(v) => *v,
            Part::Val(v) => *v,
        }
    }

    fn build_from_str(tree: &mut Tree, parent: Option<usize>, s: &str) -> Result<Self, String> {
        let res = if &s[0..1] == "[" {
            let pair = Pair::build_from_str(tree, parent, s)?;
            Part::Child(pair)
        } else {
            Part::Val(s.parse().unwrap())
        };
        Ok(res)
    }

    fn as_string(&self, arena: &Vec<Pair>) -> String {
        match self {
            Part::Child(idx) => arena[*idx].as_string(arena),
            Part::Val(v) => v.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    l: Part,
    r: Part,
    idx: usize,
    parent: Option<usize>,
}

impl Pair {
    fn build_from_str(tree: &mut Tree, parent: Option<usize>, s: &str) -> Result<usize, String> {
        let comma = s
            .char_indices()
            .fold_while(0, |acc, (i, c)| match (acc, c) {
                (1, ',') => Done(i),
                (_, '[') => Continue(acc + 1),
                (_, ']') => Continue(acc - 1),
                (_, _) => Continue(acc),
            })
            .into_inner();
        let l = &s[1..comma];
        let r = &s[comma + 1..s.len() - 1];
        let idx = tree.size();
        let mut pair = Pair {
            parent,
            idx,
            l: Part::Val(0),
            r: Part::Val(0),
        };
        tree.push(pair.clone());
        // let mut l = Part::build_from_str(tree, Some(idx), l)?;
        pair.l = Part::build_from_str(tree, Some(idx), l)?;
        pair.r = Part::build_from_str(tree, Some(idx), r)?;
        tree.arena[idx] = pair;
        Ok(idx)
    }

    fn as_string(&self, arena: &Vec<Pair>) -> String {
        format!("[{},{}]", self.l.as_string(arena), self.r.as_string(arena))
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Tree> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let input = input
        .into_iter()
        .fold1(|mut acc, tree| {
            acc += tree;
            acc
        })
        .unwrap();

    input.magnitude()
}

fn part2(input: &str) -> usize {
    let pairs = parse_input(input);
    let a = pairs
        .into_iter()
        .permutations(2)
        .map(|mut v| {
            let v1 = v.pop().unwrap();
            v[0] += v1;
            v[0].magnitude()
        })
        .max()
        .unwrap();

    let input = parse_input(input);
    let b = input
        .into_iter()
        .permutations(2)
        .map(|mut v| {
            let mut v1 = v.pop().unwrap();
            let v0 = v.pop().unwrap();
            v1 += v0;
            v1.magnitude()
        })
        .max()
        .unwrap();
    std::cmp::max(a, b)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{parse_input, part1, part2, Tree};

    const EXAMPLE: [&str; 2] = ["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"];

    const EX2: &str = r#"
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#;

    const EX3: &str = r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#;

    const EX4: &str = r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#;

    const EX5: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    #[test]
    fn test_reduce_p1() {
        let t1 = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let mut t1 = t1.parse::<Tree>().unwrap();
        t1.reduce();
        // println!("{}", t1);
        assert_eq!(t1.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    }

    #[test]
    fn test_add_p1() {
        let mut t1: Tree = EXAMPLE[0].parse().unwrap();
        let t2: Tree = EXAMPLE[1].parse().unwrap();
        // println!("{} \n {}", t1, t2);
        t1 += t2;
        // println!("{}", t1);
        assert_eq!(t1.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }

    #[test]
    fn test_first2_ex2() {
        let pairs = parse_input(EX2).into_iter().take(2).collect_vec();
        let pair = pairs
            .into_iter()
            .fold1(|mut acc, tree| {
                acc += tree;
                // println!("test: {}", acc);
                acc
            })
            .unwrap();
        assert_eq!(
            pair.to_string(),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        )
    }

    #[test]
    fn test_example2_add() {
        let pairs = parse_input(EX2);

        let pair = pairs
            .into_iter()
            .fold1(|mut acc, tree| {
                acc += tree;
                // println!("test2: {}", acc);
                acc
            })
            .unwrap();
        assert_eq!(
            pair.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )
    }

    #[test]
    fn test_example3_add() {
        let pairs = parse_input(EX3);
        let pair = pairs
            .into_iter()
            .fold1(|mut acc, tree| {
                acc += tree;
                acc
            })
            .unwrap();
        assert_eq!(pair.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]")
    }

    #[test]
    fn test_example4_add() {
        let pairs = parse_input(EX4);
        let pair = pairs
            .into_iter()
            .fold1(|mut acc, tree| {
                acc += tree;
                acc
            })
            .unwrap();
        assert_eq!(pair.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]")
    }

    #[test]
    fn test_example_5_pair() {
        let pairs = parse_input(EX5);
        let pair = pairs
            .into_iter()
            .fold1(|mut acc, tree| {
                acc += tree;
                acc
            })
            .unwrap();
        assert_eq!(
            pair.to_string(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        )
    }

    #[test]
    fn test_example_5_magnitude() {
        assert_eq!(part1(EX5), 4140)
    }

    #[test]
    fn test_magnitude() {
        let input = "[[1,2],[[3,4],5]]";
        let t1 = input.parse::<Tree>().unwrap();
        assert_eq!(t1.magnitude(), 143)
    }

    #[test]
    fn test_magnitude2() {
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let t1 = input.parse::<Tree>().unwrap();
        assert_eq!(t1.magnitude(), 1384)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX5), 3993)
    }
}
