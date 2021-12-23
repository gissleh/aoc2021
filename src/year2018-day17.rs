use common::aoc::{print_result, run_many, print_time_cold};
use common::parser;
use common::grid::FixedGrid;
use std::cmp::{min, max};

fn main() {
    let input = include_bytes!("../input/year2018-day17.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &Reservoir) -> u32 {
    let mut reservoir = input.clone();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(input.spring_pos);

    while let Some((x, y)) = stack.pop() {
        let mut found_any = false;

        let old = reservoir.grid[(x, y)];
        reservoir.grid[(x, y)] = b'@';
        reservoir.grid.print();
        reservoir.grid[(x, y)] = old;

        if reservoir.grid.has_oob(x, y, b'|', true) && reservoir.grid.has_oob(x, y + 1, b'|', true) {
            continue;
        }

        stack.push((x, y));
        if let Some(pos) = reservoir.fill(x, y, 1) {
            stack.push(pos);
            found_any = true;
        }
        if let Some(pos) = reservoir.fill(x, y, !0) {
            stack.push(pos);
            found_any = true;
        }

        if found_any {
            reservoir.settle(x, y);
        } else {
            stack.pop();
        }
    }

    let mut count = 0;
    for line in reservoir.grid.lines().skip(reservoir.min_y) {
        for v in line.iter() {
            match *v {
                b'|' | b'~' => { count += 1; }
                _ => {}
            }
        }
    }

    count
}

fn part2(input: &Reservoir) -> u32 {
    0
}

fn parse_input(input: &[u8]) -> Reservoir {
    let mut lines = Vec::with_capacity(64);
    let mut input = input;
    let mut min_x = !0usize;
    let mut max_x = 0usize;
    let mut min_y = !0usize;
    let mut max_y = 0usize;
    while let Some((line, new_input)) = BuildLine::parse(input) {
        let (bx1, by1, bx2, by2) = line.bounds();
        min_x = min(min_x, bx1);
        min_y = min(min_y, by1);
        max_x = max(max_x, bx2);
        max_y = max(max_y, by2);

        lines.push(line);
        input = new_input;
    }
    let width = (max_x - min_x) + 1;
    let height = max_y + 1;

    let mut grid = FixedGrid::new(width, height, b'.');

    for line in lines.iter() {
        for (x, y) in line.positions() {
            grid.set(x - min_x, y, b'#');
        }
    }

    Reservoir {
        grid,
        min_y,
        spring_pos: (500 - min_x, 0),
    }
}

#[derive(Clone)]
struct Reservoir {
    /// The # and . dream.
    grid: FixedGrid<u8>,
    /// The position of the spring. I truncate the grid, so it is not always
    spring_pos: (usize, usize),
    /// The grid works with y-values above it, but they shall not be counted.
    min_y: usize,
}

impl Reservoir {
    fn fill(&mut self, x: usize, y: usize, x_dir: usize) -> Option<(usize, usize)> {
        if y >= self.grid.height() - 1 {
            return None;
        }

        let mut x = x;
        while x < self.grid.width() {
            if self.grid.has(x, y, b'#') {
                break;
            }

            if self.grid.has(x, y + 1, b'.') {
                self.grid.set(x, y, b'|');
                return Some((x, y + 1));
            }

            self.grid.set(x, y, b'|');
            x = x.wrapping_add(x_dir);
            if self.grid.has(x, y, b'|') {
                break;
            }
        }

        None
    }

    fn settle(&self, src_x: usize, y: usize) {
        let mut left = x;
        let mut x = x;
        while x < self.grid.width() {
            if self.grid.has(x, y, b'~') {
                break;
            }
        }
    }

    fn is_open(&self, x: usize, y: usize) -> bool {
        self.grid.has(x, y, b'.')
    }
}

struct BuildLine {
    vertical: bool,
    fixed: usize,
    from: usize,
    to: usize,
}

impl BuildLine {
    fn positions<'a>(&'a self) -> impl Iterator<Item=(usize, usize)> + 'a {
        (self.from..=self.to).map(|v| if self.vertical {
            (self.fixed, v)
        } else {
            (v, self.fixed)
        })
    }

    fn bounds(&self) -> (usize, usize, usize, usize) {
        if self.vertical {
            (self.fixed, self.from, self.fixed, self.to)
        } else {
            (self.from, self.fixed, self.to, self.fixed)
        }
    }

    fn parse(input: &[u8]) -> Option<(BuildLine, &[u8])> {
        let (fixed_coord, input) = parser::byte(input)?;
        let (_, input) = parser::expect_byte(input, b'=')?;
        let (fixed_value, input) = parser::uint(input)?;
        let expected = if fixed_coord == b'x' { b", y=" } else { b", x=" };
        let (_, input) = parser::expect_bytes(input, expected)?;
        let (from_value, input) = parser::uint(input)?;
        let (_, input) = parser::expect_string(input, "..")?;
        let (to_value, input) = parser::uint(input)?;
        let (_, input) = parser::rest_of_line(input)?;

        if fixed_coord != b'x' && fixed_coord != b'y' {
            return None;
        }

        Some((BuildLine {
            vertical: fixed_coord == b'x',
            fixed: fixed_value,
            from: from_value,
            to: to_value,
        }, input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[u8] = b"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

    #[test]
    fn part1_works_on_example() {
        let reservoir = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&reservoir), 57);
    }
}