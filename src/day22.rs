use common::aoc::{print_result, run_many, print_time_cold};
use common::octree::{Cube, Octree, Point};
use common::parser;

fn main() {
    let input = include_bytes!("../input/day22.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[Line]) -> usize {
    let constraint = Cube(
        Point(-50, -50, -50),
        Point(51, 51, 51),
    );

    let mut octy = Octree::new(64);
    for Line(toggle, cube) in input.iter() {
        //println!("{:?}", cube.constrained(&constraint));
        if let Some(cube) = cube.constrained(&constraint) {
            match toggle {
                Toggle::On => octy.set_cube(cube, Some(())),
                Toggle::Off => octy.set_cube(cube, None),
            }
        }
    }

    octy.count(|_| true)
}

fn part2(input: &[Line]) -> usize {
    let mut octy = Octree::new(1 << 18);
    for Line(toggle, cube) in input.iter() {
        match toggle {
            Toggle::On => octy.set_cube(*cube, Some(())),
            Toggle::Off => octy.set_cube(*cube, None),
        }
    }

    octy.count(|_| true)
}

struct Line(Toggle, Cube);

enum Toggle {
    On,
    Off,
}

fn parse_line(input: &[u8]) -> Option<(Line, &[u8])> {
    let (on_off, input) = parser::word(input)?;
    let (_, input) = parser::expect_bytes(input, b"x=")?;
    let (min_x, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_x, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::expect_bytes(input, b",y=")?;
    let (min_y, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_y, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::expect_bytes(input, b",z=")?;
    let (min_z, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::expect_bytes(input, b"..")?;
    let (max_z, input) = parser::int::<isize>(input)?;
    let (_, input) = parser::rest_of_line(input)?;

    match on_off {
        b"on" => Some((Line(Toggle::On, Cube(Point(min_x, min_y, min_z), Point(max_x+1, max_y+1, max_z+1))), input)),
        b"off" => Some((Line(Toggle::Off, Cube(Point(min_x, min_y, min_z), Point(max_x+1, max_y+1, max_z+1))), input)),
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"on x=-20..26,y=-36..17,z=-47..7
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

    #[test]
    fn part1_works_on_example() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 590784);
    }
}