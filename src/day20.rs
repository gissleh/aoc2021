use common::aoc::{print_result, run_many, print_time_cold};
use common::parser;
use common::grid::FixedGrid;
use std::mem::swap;

const OFFSETS: [(usize, usize); 9] = [
    (!0, !0),
    (0, !0),
    (1, !0),
    (!0, 0),
    (0, 0),
    (1, 0),
    (!0, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let input = include_bytes!("../input/day20.txt");

    let ((enhancement, initial), dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10, || puzzle(&enhancement, &initial, 2));
    let (res_p2, dur_p2, dur_p2c) = run_many(10, || puzzle(&enhancement, &initial, 50));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn puzzle(enhancement: &[u8; 512], initial: &FixedGrid<u8>, steps: usize) -> usize {
    let width = initial.width() + ((steps) * 2);
    let height = initial.height() + ((steps) * 2);
    let mut curr_grid = FixedGrid::new(width, height, b'.');
    let mut next_grid = FixedGrid::new(width, height, b'.');
    let mut oob = false;

    curr_grid.blit(initial, steps, steps);

    // Print the grid when testing.
    #[cfg(test)] {
        println!();
        curr_grid.print();
    }

    for _ in 0..steps {
        // Perform search
        for y in 0..curr_grid.width() {
            for x in 0..curr_grid.height() {
                let mut index = 0;
                for (xo, yo) in OFFSETS.iter() {
                    let x = x.wrapping_add(*xo);
                    let y = y.wrapping_add(*yo);

                    index <<= 1;
                    if curr_grid.has_oob(x, y, b'#', oob) {
                        index |= 1;
                    }
                }

                next_grid.set(x, y, enhancement[index]);
            }
        }

        // Lazy copy next_grid to curr_grid.
        swap(&mut curr_grid, &mut next_grid);

        // For cases where the 0th enhancement is '#', invert the infinite sea beyond our bounds.
        if enhancement[0] == b'#' {
            oob = !oob;
        }

        // Testing: show the grid
        #[cfg(test)] {
            println!();
            curr_grid.print();
        }
    }

    curr_grid.data().iter().filter(|v| **v == b'#').count()
}

fn parse_input(input: &[u8]) -> ([u8; 512], FixedGrid<u8>) {
    let (enhancement, input) = parse_enhancement_data(input).unwrap();
    let (_, input) = parser::rest_of_line(input).unwrap();
    let (initial, _) = parse_initial_grid(input).unwrap();

    (enhancement, initial)
}

fn parse_enhancement_data(input: &[u8]) -> Option<([u8; 512], &[u8])> {
    if input.len() < 512 {
        return None;
    }

    let mut arr = [0u8; 512];
    arr.copy_from_slice(&input[..512]);
    if arr.iter().find(|v| **v != b'.' && **v != b'#').is_some() {
        return None;
    }

    let (_, input) = parser::rest_of_line(&input[512..])?;

    Some((arr, input))
}

fn parse_initial_grid(input: &[u8]) -> Option<(FixedGrid<u8>, &[u8])> {
    let width = input.iter().take_while(|p| **p != b'\n').count();
    let mut data = Vec::with_capacity(width * width);
    let mut prev = 0u8;
    let mut off = 0;
    for v in input.iter() {
        match *v {
            b'#' | b'.' => {
                data.push(*v);
                off += 1;
            }
            b'\n' => {
                if prev == b'\n' {
                    break;
                } else {
                    off += 1;
                }
            }
            _ => {
                return None;
            }
        }

        prev = *v;
    }

    Some((FixedGrid::from(width, data.len() / width, data), &input[off..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn part1() {
        let (enhancement, initial) = parse_input(EXAMPLE);

        assert_eq!(puzzle(&enhancement, &initial, 2), 35);
    }


    #[test]
    fn part2() {
        let (enhancement, initial) = parse_input(EXAMPLE);

        assert_eq!(puzzle(&enhancement, &initial, 50), 3351);
    }
}