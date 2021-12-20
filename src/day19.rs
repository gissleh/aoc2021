use std::ops::{Add, Neg, Sub};
use num::integer::sqrt;
use num::pow;
use common::aoc::{print_result, run_many, print_time_cold};
use common::parser;
use smallvec::SmallVec;

fn main() {
    let input = include_bytes!("../input/day19.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let ((res_p1, res_p2), dur_p1, dur_p1c) = run_many(10, || part1(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1+P2", dur_p1, dur_p1c);
    print_time_cold("Total", dur_p + dur_p1, dur_pc + dur_p1c);
}

fn part1(input: &[Scanner]) -> (usize, i32) {
    let mut scanners = Vec::from(input);
    let mut positions: Vec<Option<Point>> = vec![None; scanners.len()];
    let mut solved_count = 1;
    let mut points = Vec::new();
    let mut dead_ends = vec![false; input.len() * input.len()];

    for p in scanners[0].points.iter() {
        points.push(*p);
    }

    positions[0] = Some(Point(0, 0, 0));
    while solved_count < scanners.len() {
        for i in 0..scanners.len() {
            if positions[i].is_some() {
                continue;
            }

            for j in 0..scanners.len() {
                if i == j {
                    continue;
                }

                if let Some(pj) = positions[j] {
                    let di = i * scanners.len() + j;
                    if dead_ends[di] {
                        continue;
                    }

                    if let Some((p, o)) = scanners[j].find(&scanners[i]) {
                        scanners[i].rotate(o);
                        positions[i] = Some(pj + p);

                        let diff = pj + p;
                        for p in scanners[i].points.iter() {
                            points.push(*p + diff);
                        }
                        solved_count += 1;

                        break;
                    } else {
                        dead_ends[di] = true;
                    }
                }
            }
        }
    }

    points.sort_unstable();
    points.dedup();

    let mut max_diff = 0;
    for (i, p1) in positions.iter().map(|p| p.unwrap()).enumerate() {
        for p2 in positions.iter().skip(i + 1).map(|p| p.unwrap()) {
            let diff = (p2 - p1).manhattan();
            if diff > max_diff {
                max_diff = diff;
            }
        }
    }

    (points.len(), max_diff)
}

fn parse_input(input: &[u8]) -> Vec<Scanner> {
    let mut scanners = Vec::with_capacity(16);

    let mut input = input;
    while let Some((scanner, rest)) = Scanner::parse(input) {
        scanners.push(scanner);
        input = rest;
    }

    scanners
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scanner {
    index: usize,
    points: SmallVec<[Point; 64]>,
    fingerprints: Vec<(i32, usize, usize)>,
}

impl Scanner {
    fn rotate(&mut self, orientation: usize) {
        for p in self.points.iter_mut() {
            *p = p.rotated(orientation);
        }
    }

    fn find(&self, other: &Scanner) -> Option<(Point, usize)> {
        for (afp, ai, aj) in self.fingerprints.iter() {
            for (bfp, bi, bj) in other.fingerprints.iter() {
                if afp == bfp {
                    let a_points = [self.points[*ai], self.points[*aj]];
                    let b_points = [other.points[*bi], other.points[*bj]];

                    for o in 0..24 {
                        let b_points = [b_points[0].rotated(o), b_points[1].rotated(o)];

                        let l_diff = a_points[0] - b_points[0];
                        let r_diff = a_points[1] - b_points[1];

                        if l_diff == r_diff {
                            return Some((l_diff, o));
                        }
                    }
                }
            }
        }

        None
    }

    fn parse(input: &[u8]) -> Option<(Scanner, &[u8])> {
        let mut points: SmallVec<[Point; 64]> = SmallVec::new();

        let (_, input) = parser::expect_bytes(input, b"--- scanner ")?;
        let (index, input) = parser::uint(input)?;
        let (_, mut input) = parser::rest_of_line(input)?;

        while let Some((point, rest)) = Point::parse(input) {
            points.push(point);
            input = rest;
        }

        let (_, input) = parser::rest_of_line(input)?;

        let mut fingerprints = Vec::with_capacity((points.len() * (points.len() - 1)) / 2);
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                fingerprints.push((
                    points[i].distance(&points[j]),
                    i,
                    j,
                ));
            }
        }

        Some((Scanner { index, points, fingerprints }, input))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i32, i32, i32);

impl Point {
    fn rotated(&self, orientation: usize) -> Point {
        let Point(x, y, z) = *self;

        match orientation {
            0 => *self,
            1 => Point(-z, -y, -x),
            2 => Point(-z, -x, y),
            3 => Point(-z, x, -y),
            4 => Point(-z, y, x),
            5 => Point(-y, -z, x),
            6 => Point(-y, -x, -z),
            7 => Point(-y, x, z),
            8 => Point(-y, z, -x),
            9 => Point(-x, -z, -y),
            10 => Point(-x, -y, z),
            11 => Point(-x, y, -z),
            12 => Point(-x, z, y),
            13 => Point(x, -z, y),
            14 => Point(x, -y, -z),
            15 => Point(x, z, -y),
            16 => Point(y, -z, -x),
            17 => Point(y, -x, z),
            18 => Point(y, x, -z),
            19 => Point(y, z, x),
            20 => Point(z, -y, x),
            21 => Point(z, -x, -y),
            22 => Point(z, x, y),
            23 => Point(z, y, -x),
            _ => panic!("Invalid rotation {}", orientation),
        }
    }

    fn distance(&self, rhs: &Point) -> i32 {
        sqrt(
            pow(rhs.0 - self.0, 2)
                + pow(rhs.1 - self.1, 2)
                + pow(rhs.2 - self.2, 2)
        )
    }

    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn parse(input: &[u8]) -> Option<(Point, &[u8])> {
        let (x, input) = parser::int(input)?;
        let (_, input) = parser::expect_byte(input, b',')?;
        let (y, input) = parser::int(input)?;
        let (_, input) = parser::expect_byte(input, b',')?;
        let (z, input) = parser::int(input)?;
        let (_, input) = parser::rest_of_line(input)?;

        Some((Point(x, y, z), input))
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scanner() {
        let (scanner, _) = Scanner::parse(PARSE_SAMPLE).unwrap();

        assert_eq!(scanner.index, 443);
        assert_eq!(scanner.points.as_slice(), &[
            Point(404, -588, -901),
            Point(528, -643, 409),
            Point(-838, 591, 734),
            Point(390, -675, -793),
        ]);
    }

    #[test]
    fn test_parse_input() {
        let scanners = parse_input(SAMPLE_1);

        assert_eq!(scanners.len(), 5);
        assert_eq!(scanners[3].points.len(), 25);
        assert_eq!(scanners[4].points.len(), 26);
    }

    #[test]
    fn test_rotations() {
        let scanners = parse_input(ROTATION_SAMPLE);

        for scanner in scanners.iter() {
            let mut found_any = false;
            for o in 0..24 {
                let mut failed = false;
                for i in 0..scanner.points.len() {
                    if scanner.points[i].rotated(o) != scanners[0].points[i] {
                        failed = true;
                        break;
                    }
                }

                if !failed {
                    found_any = true;
                    break;
                }
            }

            assert!(found_any);
        }
    }

    #[test]
    fn test_overlap() {
        let scanners = parse_input(SAMPLE_1);

        assert_eq!(
            scanners[0].find(&scanners[1]),
            Some((
                Point(68, -1246, -43),
                11,
            ))
        );
    }

    #[test]
    fn test_both_parts() {
        let scanners = parse_input(SAMPLE_1);

        assert_eq!(part1(&scanners), (79, 3621));
    }

    const SAMPLE_1: &[u8] = b"--- scanner 0 ---
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
30,-46,-14
";

    const PARSE_SAMPLE: &[u8] = b"--- scanner 443 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
";

    const ROTATION_SAMPLE: &[u8] = b"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8
";
}


