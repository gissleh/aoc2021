use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::FixedGrid;

fn main() {
    let input = include_bytes!("../input/day25.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(50, || part1(&input));

    print_result("P1", res_p1);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("Total", dur_p + dur_p1, dur_pc + dur_p1c);
}

fn part1(input: &FixedGrid<Space>) -> u32 {
    let mut current = input.clone();
    let mut moves = Vec::with_capacity(64);

    for n in 1.. {
        for (x, y, v) in current.iter() {
            if let Space::EastCucumber = v {
                if let Space::Free = current.get_wrapping(x + 1, y) {
                    moves.push((x, y));
                }
            }
        }

        let any_moved_right = !moves.is_empty();
        for (x, y) in moves.drain(0..) {
            current.swap_wrapping((x, y), (x + 1, y));
        }

        for (x, y, v) in current.iter() {
            if let Space::SouthCucumber = v {
                if let Space::Free = current.get_wrapping(x, y + 1) {
                    moves.push((x, y));
                }
            }
        }

        let any_moved_down = !moves.is_empty();
        for (x, y) in moves.drain(0..) {
            current.swap_wrapping((x, y), (x, y + 1));
        }

        if !any_moved_down && !any_moved_right {
            return n;
        }
    }

    unreachable!()
}

fn parse_input(input: &[u8]) -> FixedGrid<Space> {
    FixedGrid::parse_bytes(input, |v| {
        match v {
            b'.' => Space::Free,
            b'>' => Space::EastCucumber,
            b'v' | b'V' => Space::SouthCucumber,
            _ => panic!("Unknown byte: {}", v as char),
        }
    })
}

#[derive(Copy, Clone)]
enum Space {
    EastCucumber,
    SouthCucumber,
    Free,
}