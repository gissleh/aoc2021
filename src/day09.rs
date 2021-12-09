use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::FixedGrid;

const OFFSETS: [(usize, usize); 4] = [
    (!0, 0),
    (0, !0),
    (1, 0),
    (0, 1),
];

fn main() {
    let input = include_bytes!("../input/day09.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let ((res_p1, low_points), dur_p1, dur_p1c) = run_many(1000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || part2(&input, &low_points));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &FixedGrid<u8>) -> (u64, Vec<(usize, usize)>) {
    let mut sum = 0;
    let mut points = Vec::with_capacity(1024);
    'outer: for (x, y, v) in input.iter() {
        for (xo, yo) in OFFSETS.iter() {
            if let Some(v2) = input.get_safe(x.wrapping_add(*xo), y.wrapping_add(*yo)) {
                if *v >= *v2 {
                    continue 'outer;
                }
            }
        }

        sum += (*v as u64) + 1;
        points.push((x, y));
    }

    (sum, points)
}

fn part2(input: &FixedGrid<u8>, low_points: &[(usize, usize)]) -> u32 {
    let mut ff_grid = FixedGrid::new(input.width(), input.height(), false);
    let mut biggest_basins = Vec::with_capacity(4);
    let mut stack = Vec::with_capacity(1024);

    for (x, y) in low_points.iter().cloned() {
        let mut filled = 0;
        stack.clear();
        stack.push((x, y));

        while let Some((x, y)) = stack.pop() {
            if *ff_grid.get(x, y).unwrap() == true || *input.get(x, y).unwrap() == 9  {
                continue;
            }

            filled += 1;
            ff_grid.set(x, y, true);

            for (xo, yo) in OFFSETS.iter() {
                let x2 = x.wrapping_add(*xo);
                let y2 = y.wrapping_add(*yo);
                if x2 < input.width() && y2 < input.height() {
                    stack.push((x2, y2));
                }
            }
        }

        biggest_basins.push(filled);
        if biggest_basins.len() > 3 {
            let mut smallest_basin_index = 0;
            for i in 1..4 {
                if biggest_basins[i] < biggest_basins[smallest_basin_index] {
                    smallest_basin_index = i;
                }
            }

            biggest_basins.swap_remove(smallest_basin_index);
        }
    }

    biggest_basins.iter().product()
}

fn parse_input(input: &[u8]) -> FixedGrid<u8> {
    let data: Vec<u8> = input.iter().filter(|v| **v != b'\n').map(|v| *v - b'0').collect();
    let width = input.iter().take_while(|p| **p != b'\n').count();
    let height = data.len() / width;

    FixedGrid::from(width, height, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = b"2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part1() {
        let grid = parse_input(SAMPLE);

        assert_eq!(part1(&grid).0, 15);
    }

    #[test]
    fn test_part2() {
        let grid = parse_input(SAMPLE);
        let (_, low_points) = part1(&grid);

        assert_eq!(part2(&grid, &low_points), 1134);
    }
}