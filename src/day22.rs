use std::cmp::{max, min};
use std::mem::swap;
use common::aoc::{print_result, run_many, print_time_cold};
use common::octree::{IndexCube, Octree, IndexPoint};
use common::parser;
use std::ops::{Sub, Add};
use smallvec::{SmallVec, smallvec};

fn main() {
    let input = include_bytes!("../input/day22.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100, || part1_octree(&input));
    let (res_p1_c, dur_p1_c, dur_p1c_c) = run_many(100, || part1_cubes(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1, || part2_octree(&input));
    let (res_p2_c, dur_p2_c, dur_p2c_c) = run_many(100, || part2_cubes(&input));

    print_result("P1 (Octree)", res_p1);
    print_result("P1 (Cubes)", res_p1_c);
    print_result("P2 (Octree)", res_p2);
    print_result("P2 (Cubes)", res_p2_c);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1 (Octree)", dur_p1, dur_p1c);
    print_time_cold("P1 (Cubes)", dur_p1_c, dur_p1c_c);
    print_time_cold("P2 (Octree)", dur_p2, dur_p2c);
    print_time_cold("P2 (Cubes)", dur_p2_c, dur_p2c_c);
    print_time_cold("Total (Cubes)", dur_p + dur_p1_c + dur_p2_c, dur_pc + dur_p1c_c + dur_p2c_c);
}

fn part1_cubes(input: &[Line]) -> i64 {
    let mut cubes: Vec<Cuboid> = Vec::with_capacity(1024);
    let mut next_cubes: Vec<Cuboid> = Vec::with_capacity(1024);

    let constraint = Cuboid {
        min: CuboidPoint([-50, -50, -50]),
        max: CuboidPoint([51, 51, 51]),
    };

    for Line(toggle, cube) in input.iter() {
        if let Some(cube) = cube.constrained(&constraint) {
            next_cubes.clear();

            for c in cubes.iter() {
                next_cubes.extend_from_slice(&c.subtract(&cube));
            }

            if let Toggle::On = toggle {
                next_cubes.push(cube);
            }

            swap(&mut cubes, &mut next_cubes)
        }
    }

    cubes.iter().map(|c| c.volume()).sum()
}

fn part1_octree(input: &[Line]) -> usize {
    let constraint = IndexCube(
        IndexPoint(-50, -50, -50),
        IndexPoint(51, 51, 51),
    );

    let mut octy = Octree::new(64, None);
    for Line(toggle, cube) in input.iter() {
        //println!("{:?}", cube.constrained(&constraint));
        if let Some(cube) = cube.octree_key().constrained(&constraint) {
            match toggle {
                Toggle::On => octy.set_cube(cube, Some(())),
                Toggle::Off => octy.set_cube(cube, None),
            }
        }
    }

    octy.count(|_| true)
}

fn part2_cubes(input: &[Line]) -> i64 {
    let mut cubes: Vec<Cuboid> = Vec::with_capacity(1024);
    let mut next_cubes: Vec<Cuboid> = Vec::with_capacity(1024);

    for Line(toggle, cube) in input.iter() {
        next_cubes.clear();

        for c in cubes.iter() {
            next_cubes.extend_from_slice(&c.subtract(cube));
        }

        if let Toggle::On = toggle {
            next_cubes.push(*cube);
        }

        swap(&mut cubes, &mut next_cubes)
    }

    cubes.iter().map(|c| c.volume()).sum()
}

#[allow(dead_code)]
fn part2_octree(input: &[Line]) -> usize {
    let mut octy = Octree::new(131072, None);

    for Line(toggle, cube) in input.iter() {
        match toggle {
            Toggle::On => octy.set_cube(cube.octree_key(), Some(())),
            Toggle::Off => octy.set_cube(cube.octree_key(), None),
        }
    }

    octy.count(|_| true)
}

struct Line(Toggle, Cuboid);

enum Toggle {
    On,
    Off,
}

fn parse_line(input: &[u8]) -> Option<(Line, &[u8])> {
    let (on_off, input) = parser::word(input)?;
    let (_, input) = parser::expect_bytes(input, b"x=")?;
    let (min_x, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_x, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::expect_bytes(input, b",y=")?;
    let (min_y, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_y, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::expect_bytes(input, b",z=")?;
    let (min_z, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_z, input) = parser::int::<i64>(input)?;
    let (_, input) = parser::rest_of_line(input)?;

    let cuboid = Cuboid {
        min: CuboidPoint([min_x, min_y, min_z]),
        max: CuboidPoint([max_x + 1, max_y + 1, max_z + 1]),
    };

    match on_off {
        b"on" => Some((Line(Toggle::On, cuboid), input)),
        b"off" => Some((Line(Toggle::Off, cuboid), input)),
        _ => None
    }
}

fn parse_input(input: &[u8]) -> Vec<Line> {
    let mut lines = Vec::with_capacity(64);
    let mut input = input;
    while let Some((line, remainder)) = parse_line(input) {
        lines.push(line);
        input = remainder;
    }

    lines
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Cuboid {
    min: CuboidPoint,
    max: CuboidPoint,
}

impl Cuboid {
    fn volume(&self) -> i64 {
        (self.max - self.min).0.iter().product()
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        (self.max.x() > other.min.x())
            && (self.min.x() < other.max.x())
            && (self.max.y() > other.min.y())
            && (self.min.y() < other.max.y())
            && (self.max.z() > other.min.z())
            && (self.min.z() < other.max.z())
    }

    fn subtract(&self, other: &Cuboid) -> SmallVec<[Cuboid; 9]> {
        if self.overlaps(other) {
            let mut list = self.split_by(other);
            list.pop();
            list
        } else {
            smallvec![*self]
        }
    }

    /// split_by splits the cube into upto 9 slices. The last slice will be the one that
    /// overlaps the other cube.
    fn split_by(&self, other: &Cuboid) -> SmallVec<[Cuboid; 9]> {
        if self.overlaps(other) {
            let mut cubes = SmallVec::new();
            let mut middle = *self;
            for i in 0..3 {
                let slices = middle.slash_twice(i, other.min.0[i], other.max.0[i]);
                for slice in slices.iter().skip(1) {
                    if !slice.is_flat() {
                        cubes.push(*slice);
                    }
                }
                middle = slices[0];
            }

            if !middle.is_flat() {
                cubes.push(middle);
            }

            cubes
        } else {
            smallvec![*self]
        }
    }

    // slash_twice returns 1-3 cubes. The first will always be the "middle" cube.
    fn slash_twice(&self, i: usize, c1: i64, c2: i64) -> SmallVec<[Cuboid; 3]> {
        let (a, b) = self.slash(i, c1);
        if let Some(b) = b {
            let (b, c) = b.slash(i, c2);
            if let Some(c) = c {
                smallvec![b, a, c]
            } else {
                smallvec![b, a]
            }
        } else {
            let (b, c) = a.slash(i, c2);
            if let Some(c) = c {
                smallvec![b, c]
            } else {
                smallvec![a]
            }
        }
    }

    fn slash(&self, i: usize, c: i64) -> (Cuboid, Option<Cuboid>) {
        if c <= self.min.0[i] || c >= self.max.0[i] {
            (*self, None)
        } else {
            let mut a = *self;
            let mut b = *self;

            a.max.0[i] = c;
            b.min.0[i] = c;

            (a, Some(b))
        }
    }

    fn is_flat(&self) -> bool {
        for i in 0..3 {
            if self.min.0[i] == self.max.0[i] {
                return true;
            }
        }

        false
    }

    fn octree_key(&self) -> IndexCube {
        IndexCube(
            IndexPoint(self.min.x() as isize, self.min.y() as isize, self.min.z() as isize),
            IndexPoint(self.max.x() as isize, self.max.y() as isize, self.max.z() as isize),
        )
    }

    fn constrained(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.overlaps(other) {
            Some(Cuboid {
                min: CuboidPoint([
                    max(self.min.0[0], other.min.0[0]),
                    max(self.min.0[1], other.min.0[1]),
                    max(self.min.0[2], other.min.0[2]),
                ]),
                max: CuboidPoint([
                    min(self.max.0[0], other.max.0[0]),
                    min(self.max.0[1], other.max.0[1]),
                    min(self.max.0[2], other.max.0[2]),
                ]),
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CuboidPoint([i64; 3]);

impl CuboidPoint {
    fn x(&self) -> i64 {
        self.0[0]
    }

    fn y(&self) -> i64 {
        self.0[1]
    }

    fn z(&self) -> i64 {
        self.0[2]
    }
}

impl Sub for CuboidPoint {
    type Output = CuboidPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        CuboidPoint([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2] - rhs.0[2]])
    }
}

impl Add for CuboidPoint {
    type Output = CuboidPoint;

    fn add(self, rhs: Self) -> Self::Output {
        CuboidPoint([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_by_conserves_volume() {
        let cuboid = Cuboid {
            min: CuboidPoint([-14, -13, -12]),
            max: CuboidPoint([17, 33, 23]),
        };

        for x in -15..15 {
            for y in -15..15 {
                for z in -15..15 {
                    let cuboid2 = Cuboid {
                        min: CuboidPoint([x, y, z]),
                        max: CuboidPoint([x + 15, y + 15, z + 15]),
                    };

                    let cuboids = cuboid.split_by(&cuboid2);
                    assert_eq!(cuboids.iter().map(|c| c.volume()).sum::<i64>(), cuboid.volume());

                    for (i, c1) in cuboids.iter().enumerate() {
                        for c2 in cuboids.iter().skip(i + 1) {
                            assert_eq!(c1.overlaps(c2), false);
                        }
                        println!("\n{:?}\n{:?}", c1, cuboid);
                        assert_eq!(c1.overlaps(&cuboid), true);
                    }
                }
            }
        }
    }

    #[test]
    fn split_by_corner() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([15, 15, 15]),
        };
        let c2 = Cuboid {
            min: CuboidPoint([13, 13, 13]),
            max: CuboidPoint([17, 17, 17]),
        };

        let split_cubes = c1.split_by(&c2);

        assert_eq!(split_cubes.len(), 4);
        assert_eq!(split_cubes.as_slice(), [
            Cuboid { min: CuboidPoint([10, 10, 10]), max: CuboidPoint([13, 15, 15]) },
            Cuboid { min: CuboidPoint([13, 10, 10]), max: CuboidPoint([15, 13, 15]) },
            Cuboid { min: CuboidPoint([13, 13, 10]), max: CuboidPoint([15, 15, 13]) },
            Cuboid { min: CuboidPoint([13, 13, 13]), max: CuboidPoint([15, 15, 15]) },
        ].as_slice())
    }

    #[test]
    fn slash_twice_middle() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([15, 15, 15]),
        };

        let split = c1.slash_twice(0, 12, 14);
        assert_eq!(split.as_slice(), [
            Cuboid {
                min: CuboidPoint([12, 10, 10]),
                max: CuboidPoint([14, 15, 15]),
            },
            Cuboid {
                min: CuboidPoint([10, 10, 10]),
                max: CuboidPoint([12, 15, 15]),
            },
            Cuboid {
                min: CuboidPoint([14, 10, 10]),
                max: CuboidPoint([15, 15, 15]),
            },
        ].as_slice());
    }

    #[test]
    fn slash_twice_side() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([15, 15, 15]),
        };

        let split = c1.slash_twice(0, 12, 16);
        assert_eq!(split.as_slice(), [
            Cuboid {
                min: CuboidPoint([12, 10, 10]),
                max: CuboidPoint([15, 15, 15]),
            },
            Cuboid {
                min: CuboidPoint([10, 10, 10]),
                max: CuboidPoint([12, 15, 15]),
            },
        ].as_slice());
    }

    #[test]
    fn slash_twice_nonoverlapping() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([15, 15, 15]),
        };

        let split = c1.slash_twice(0, 22, 32);
        assert_eq!(split.as_slice(), [
            Cuboid {
                min: CuboidPoint([10, 10, 10]),
                max: CuboidPoint([15, 15, 15]),
            }
        ].as_slice());
    }

    #[test]
    fn slash_twice_adjacent() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([15, 15, 15]),
        };

        let split = c1.slash_twice(0, 15, 32);
        assert_eq!(split.as_slice(), [
            Cuboid {
                min: CuboidPoint([10, 10, 10]),
                max: CuboidPoint([15, 15, 15]),
            }
        ].as_slice());

        let split = c1.slash_twice(0, 0, 10);
        assert_eq!(split.as_slice(), [
            Cuboid {
                min: CuboidPoint([10, 10, 10]),
                max: CuboidPoint([15, 15, 15]),
            }
        ].as_slice());
    }

    #[test]
    fn subtract_once_works() {
        let c1 = Cuboid {
            min: CuboidPoint([10, 10, 10]),
            max: CuboidPoint([13, 13, 13]),
        };
        let c2 = Cuboid {
            min: CuboidPoint([11, 11, 11]),
            max: CuboidPoint([14, 14, 14]),
        };

        for c in c1.subtract(&c2) {
            println!("{:?} {}", c, c.volume());
        }

        assert_eq!(
            c1.subtract(&c2).iter()
                .map(|v| v.volume())
                .sum::<i64>(),
            19,
        );
    }

    #[test]
    fn subtract_corner() {
        let c1 = Cuboid {
            min: CuboidPoint([11, 11, 11]),
            max: CuboidPoint([14, 14, 14]),
        };
        let c2 = Cuboid {
            min: CuboidPoint([9, 9, 9]),
            max: CuboidPoint([12, 12, 12]),
        };

        println!("{:?}", c1.split_by(&c2));
        println!("{:?}", c2.split_by(&c1));

        assert_eq!(c1.split_by(&c2).len(), 4);
    }

    #[test]
    fn slash_conserves_volume() {
        let cuboid = Cuboid {
            min: CuboidPoint([-14, -13, -12]),
            max: CuboidPoint([17, 33, 23]),
        };
        let target = cuboid.volume();

        for z in -30..30 {
            let (cube, cube2) = cuboid.slash(2, z);
            if let Some(cube2) = cube2 {
                assert_eq!(cube2.volume() + cube.volume(), target);
            } else {
                assert_eq!(cube.volume(), target);
            }
        }
    }

    #[test]
    fn part1_octree_works_on_example() {
        let input = parse_input(BIG_EXAMPLE_1);
        assert_eq!(part1_octree(&input), 590784);
    }

    #[test]
    fn part1_cubes_works_on_example() {
        let input = parse_input(BIG_EXAMPLE_1);
        assert_eq!(part1_cubes(&input), 590784);
    }

    #[test]
    fn part1_cubes_works_on_second_example() {
        let input = parse_input(BIG_EXAMPLE_2);
        assert_eq!(part1_cubes(&input), 474140);
    }

    #[test]
    fn part1_cubes_works_on_simple_example() {
        let input = parse_input(SIMPLE_EXAMPLE_1);
        assert_eq!(part1_cubes(&input[..1]), 27);
        assert_eq!(part1_cubes(&input[..2]), 46);
        assert_eq!(part1_cubes(&input[..3]), 38);
        assert_eq!(part1_cubes(&input), 39);
    }

    #[test]
    fn part2_cubes_works_on_example() {
        let input = parse_input(BIG_EXAMPLE_2);
        assert_eq!(part2_cubes(&input), 2758514936282235);
    }

    const BIG_EXAMPLE_1: &[u8] = b"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

    const BIG_EXAMPLE_2: &[u8] = b"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
";

    const SIMPLE_EXAMPLE_1: &[u8] = b"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
";
}