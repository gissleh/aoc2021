use std::cmp::max;
use num::abs;
use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::FixedGrid;
use common::parsers::{parse_u32s_amount};

const RIGHT: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (-1, 0);

fn main() {
    let input = include_bytes!("../input/day05.txt");

    let ((lines, (width, height)), dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || puzzle(&lines, width, height, false));
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || puzzle(&lines, width, height, true));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn puzzle(lines: &[Line], width: usize, height: usize, allow_diagonals: bool) -> usize {
    let mut grid = FixedGrid::new(width, height, 0);
    for Line{source: (sx, sy), dir: (dx, dy), len} in lines.iter() {
        if !allow_diagonals && (*dx != 0 && *dy != 0) {
            continue
        }

        if (*dx, *dy) == RIGHT {
            let slice = grid.get_slice_mut(*sx as usize,*sx as usize + len + 1, *sy as usize);
            for v in slice {
                *v += 1;
            }
        } else if (*dx, *dy) == LEFT {
            let slice = grid.get_slice_mut(*sx as usize - len, *sx as usize + 1, *sy as usize);
            for v in slice {
                *v += 1;
            }
        }else {
            let mut x = *sx;
            let mut y = *sy;
            for _ in 0..=*len {
                *grid.get_mut(x as usize, y as usize).unwrap() += 1;

                x += dx;
                y += dy;
            }
        }
    }

    grid.data().iter().filter(|v| **v > 1).count()
}

fn parse_input(input: &[u8]) -> (Vec<Line>, (usize, usize)) {
    let mut current_line = Vec::with_capacity(4);
    let mut res = Vec::with_capacity(100);
    let mut pos = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    loop {
        current_line.clear();
        pos += parse_u32s_amount(&input[pos..], &mut current_line, 4);
        if current_line.len() < 4 {
            break;
        }

        let (x1, y1, x2, y2) = (
            current_line[0] as i32,
            current_line[1] as i32,
            current_line[2] as i32,
            current_line[3] as i32
        );
        let dist = max(abs(x2 - x1), abs(y2 - y1));

        res.push(Line{
            source: (x1, y1),
            dir: ((x2-x1)/dist, (y2-y1)/dist),
            len: dist as usize,
        });

        if x1 > max_x {
            max_x = x1
        }
        if x2 > max_x {
            max_x = x2
        }
        if y1 > max_y {
            max_y = y1
        }
        if y2 > max_y {
            max_y = y2
        }
    }

    (res, ((max_x+1) as usize, (max_y+1) as usize))
}

#[derive(Debug)]
struct Line {
    source: (i32, i32),
    dir: (i32, i32),
    len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_part1() {
        let (lines, (width, height)) = parse_input(SAMPLE_INPUT.as_bytes());
        assert_eq!(puzzle(&lines, width, height, false), 5);
    }

    #[test]
    fn test_part2() {
        let (lines, (width, height)) = parse_input(SAMPLE_INPUT.as_bytes());
        assert_eq!(puzzle(&lines, width, height, true), 12);
    }
}