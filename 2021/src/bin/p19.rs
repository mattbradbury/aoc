use aoc_helper::load_input;
use itertools::Itertools;
use na::{Matrix3, Point3};
use nalgebra as na;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    time,
};

fn main() {
    let input = load_input(2021, 19);
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

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// struct Point3 {
//     loc: [isize; 3],
// }

// impl FromStr for Point3 {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let loc  = s.trim().split(',').map(|s| s.parse().unwrap()).collect_vec().as_slice().try_into().unwrap();
//         Ok(Self {loc})
//     }
// }

// impl Sub for Point3 {
//     type Output = Point3;

//     fn sub(self, rhs: Self) -> Self::Output {
//         let mut loc = [0; 3];
//         for i in 0..3 {
//             loc[i] = self.loc[i] - rhs.loc[i];
//         };
//         Point3 { loc }
//     }
// }

// impl Index<usize> for Point3 {
//     type Output=isize;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.loc[index]
//     }
// }

#[derive(Debug, Clone, Copy)]
struct Triangle {
    points: [na::Point3<isize>; 3],
    area: isize, // fake area not square rooted
    sector: usize,
}

impl std::hash::Hash for Triangle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.points.hash(state);
        self.area.hash(state);
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
}

impl Eq for Triangle {}

impl Triangle {
    fn new(points: [na::Point3<isize>; 3], sector: usize) -> Self {
        let u = points[1] - points[0];
        let v = points[2] - points[0];
        let area = isize::pow(u[1] * v[2] - u[2] * v[1], 2)
            + isize::pow(u[2] * v[0] - u[0] * v[2], 2)
            + isize::pow(u[0] * v[1] - u[1] * v[0], 2);
        let area = area;
        Self {
            points,
            area,
            sector,
        }
    }

    fn align_points(&mut self, rhs: &Triangle) {
        let _og = self.points;
        let mut ret = [Point3::<isize>::origin(); 3];
        let rab = Triangle::distance2(rhs.points[0], rhs.points[1]);
        let rac = Triangle::distance2(rhs.points[0], rhs.points[2]);
        let rbc = Triangle::distance2(rhs.points[1], rhs.points[2]);

        let lab = Triangle::distance2(self.points[0], self.points[1]);
        let lac = Triangle::distance2(self.points[0], self.points[2]);
        let lbc = Triangle::distance2(self.points[1], self.points[2]);
        if lab + lac == rab + rac {
            ret[0] = self.points[0]
        } else if lab + lbc == rab + rac {
            ret[0] = self.points[1]
        } else if lac + lbc == rab + rac {
            ret[0] = self.points[2]
        };

        if lab + lac == rab + rbc {
            ret[1] = self.points[0]
        } else if lab + lbc == rab + rbc {
            ret[1] = self.points[1]
        } else if lac + lbc == rab + rbc {
            ret[1] = self.points[2]
        };

        if lab + lac == rac + rbc {
            ret[2] = self.points[0]
        } else if lab + lbc == rac + rbc {
            ret[2] = self.points[1]
        } else if lac + lbc == rac + rbc {
            ret[2] = self.points[2]
        };
        self.points = ret;
    }

    fn distance2(a: na::Point3<isize>, b: na::Point3<isize>) -> isize {
        let ab = a - b;
        let d = ab[0].pow(2) + ab[1].pow(2) + ab[2].pow(3);
        d
    }
}

fn distance2(a: na::Point3<isize>, b: na::Point3<isize>) -> isize {
    let ab = a - b;
    let ab: Point3<isize> = ab.into();
    let d = ab[0].abs().pow(2) + ab[1].abs().pow(2) + ab[2].abs().pow(2);

    // println!("{}", ab);
    d
}

fn distance(a: na::Point3<isize>, b: na::Point3<isize>) -> isize {
    let ab = a - b;
    let ab: Point3<isize> = ab.into();
    let d = ab[0].abs() + ab[1].abs() + ab[2].abs();

    // println!("{}", ab);
    d
}

#[allow(dead_code)]
fn parse_input(input: &str) -> VecDeque<HashSet<na::Point3<isize>>> {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let (x, y, z) = line
                        .split(',')
                        .map(|v| v.parse::<isize>().unwrap())
                        .take(3)
                        .collect_tuple()
                        .unwrap();
                    [x, y, z].into()
                })
                .collect()
        })
        .collect()
}

fn all_rotations() -> Vec<Matrix3<isize>> {
    let a: [[isize; 9]; 3] = [
        [1, 0, 0, 0, 1, 0, 0, 0, 1],
        [0, 1, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 1, 0],
    ];

    let b: [[isize; 9]; 4] = [
        [1, 0, 0, 0, 1, 0, 0, 0, 1],
        [-1, 0, 0, 0, -1, 0, 0, 0, 1],
        [-1, 0, 0, 0, 1, 0, 0, 0, -1],
        [1, 0, 0, 0, -1, 0, 0, 0, -1],
    ];

    let c: [[isize; 9]; 2] = [[1, 0, 0, 0, 1, 0, 0, 0, 1], [0, 0, -1, 0, -1, 0, -1, 0, 0]];

    let a: Vec<Matrix3<isize>> = a
        .iter()
        .map(|arr| Matrix3::<isize>::from_row_slice(arr))
        .collect_vec();
    let b: Vec<Matrix3<isize>> = b
        .iter()
        .map(|arr| Matrix3::<isize>::from_row_slice(arr))
        .collect_vec();
    let c: Vec<Matrix3<isize>> = c
        .iter()
        .map(|arr| Matrix3::<isize>::from_row_slice(arr))
        .collect_vec();

    let mut rotations = Vec::new();

    for i in 0..3 {
        for j in 0..4 {
            for k in 0..2 {
                rotations.push(a[i] * b[j] * c[k])
            }
        }
    }
    //   rotations.remove(0);
    rotations
}

#[allow(dead_code)]
fn triangle_set(scans: &HashSet<na::Point3<isize>>, sector: usize) -> HashSet<Triangle> {
    let res = scans
        .clone()
        .drain()
        .combinations(3)
        .map(|points| {
            let points = points.as_slice().try_into().unwrap();
            Triangle::new(points, sector)
        })
        .collect::<HashSet<Triangle>>();
    res
}

#[allow(dead_code)]
fn part1_bad(input: &str) -> usize {
    let input = parse_input(input);
    let mut points = input.iter().cloned().enumerate().collect::<HashMap<_, _>>();
    let mut sets = input
        .iter()
        .enumerate()
        .map(|(i, t)| (i, triangle_set(t, i)))
        .collect_vec();
    let _count = 0;
    let mut scanners = HashMap::new();
    scanners.insert(0, Point3::<isize>::origin());

    'outer: while sets.len() > 1 {
        let scan_num = 0;
        let mut tmap = HashMap::new();

        for j in 1..sets.len() {
            // scan_num = sets[j].0;
            dbg!(j);

            let ab = sets[0].1.intersection(&sets[j].1).collect_vec();
            if ab.len() < 220 {
                continue;
            }
            println!("set {} overlap {} ", scan_num, ab.len());
            sets[0].1.intersection(&sets[j].1).for_each(|inter| {
                tmap.entry(*inter).or_insert(Vec::new()).push(*inter);
            });
            sets[j].1.intersection(&sets[0].1).for_each(|inter| {
                tmap.entry(*inter).or_insert(Vec::new()).push(*inter);
            });
            // };

            // tmap.values().for_each(|v| println!("{}", v.len()));
            let overlaps = tmap.values().collect_vec();
            for overlap in overlaps {
                // dbg!(overlaps[0]);
                let (a, mut b) = (overlap[0], overlap[1]);
                b.align_points(&a);
                let rotations = all_rotations();
                for (_i, r) in rotations.iter().enumerate() {
                    let mut c = b.points;
                    for j in 0..2 {
                        c[j] = r * b.points[j];
                    }
                    if a.points[0] - c[0] == a.points[1] - c[1] {
                        println!("Found: rotation {:?}", r);
                        let scanner = a.points[0] - c[0];
                        // let scanner = scanner as Point3<isize> ;
                        scanners.insert(scan_num, scanner.into());
                        dbg!(a.points[0] - c[0]);
                        let clones = points[&scan_num]
                            .iter()
                            .map(|p| r * *p + scanner)
                            .collect::<HashSet<_>>();
                        points.get_mut(&0).unwrap().extend(clones.iter());
                        let tris = triangle_set(&points[&0], 0);
                        points.remove(&scan_num);
                        sets[0] = (0, tris);

                        sets.retain(|(i, _)| *i != scan_num);
                        continue 'outer;
                    }
                }
            }
        }
    }
    // dbg!(a,b);
    points[&0].len()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let rotations = all_rotations();
    let mut prints = input
        .into_iter()
        .map(|set| {
            let fpset = set
                .iter()
                .combinations(2)
                .map(|pair| distance2(*pair[0], *pair[1]))
                .collect::<HashSet<_>>();
            (set, fpset)
        })
        .collect::<VecDeque<_>>();
    let mut base = prints.pop_front().unwrap();
    dbg!(&base);
    let mut scanners = Vec::new();
    scanners.push(Point3::<isize>::origin());

    'outer: while let Some(candidate) = prints.pop_front() {
        let mutual = base.1.intersection(&candidate.1).count();
        // println!("Mutual {} {} {} {}", mutual, prints.len(), candidate.1.len(), base.1.len());
        if mutual < 66 && prints.len() > 2 {
            prints.push_back(candidate);
            continue;
        };
        let base_clone = base.clone();
        for bp in &base_clone.0 {
            for cp in &candidate.0 {
                for r in &rotations {
                    // let rotated = r * cp;
                    let diff = bp - (r * cp);
                    // let mut count = 0;
                    let moved = candidate
                        .0
                        .iter()
                        .map(|p| r * p + diff)
                        .collect::<HashSet<_>>();
                    if moved.intersection(&base.0).count() >= 12 {
                        base.0.extend(moved.iter());
                        base.1 = base
                            .0
                            .iter()
                            .combinations(2)
                            .map(|pair| {
                                assert_eq!(pair.len(), 2);
                                distance2(*pair[0], *pair[1])
                            })
                            .collect();
                        // base.1.extend(candidate.1);
                        scanners.push(diff.into());
                        println!("Scanner found {}", diff);
                        continue 'outer;
                    }
                    // for bp2 in &base_clone {
                    //     for cp2 in &candidate {
                    //         if bp2 - ( r * cp2) == diff { count += 1 }
                    //         if count >= 12 {
                    //             println!("Found rotation");
                    //             let rotated = candidate
                    //                 .iter()
                    //                 .cloned()
                    //                 .map(|p| r * p + diff);

                    //             base.extend(rotated);
                    //             scanners.push(diff.into());
                    //             continue 'outer;
                    //         }
                    //     }
                    // }
                }
            }
        }
        prints.push_back(candidate);
    }
    let maxdist = scanners
        .iter()
        .combinations(2)
        .map(|v| distance(*v[0], *v[1]))
        .max()
        .unwrap();
    println!("Scanner distance: {}", maxdist);
    base.0.len()
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const EXAMPLE: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;

    #[test]
    fn test_example1_p1() {
        assert_eq!(part1(EXAMPLE), 79)
    }

    #[test]
    fn test_triangles() {
        let _input = parse_input(EXAMPLE);
    }
    #[test]
    fn test_example1_p2() {
        assert_eq!(part2(EXAMPLE), 0)
    }
}
