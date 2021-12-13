use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_u32_pair, parse_u32b};
use common::grid::FixedGrid;

fn main() {
    let input = include_bytes!("../input/day13.txt");

    let ((points, folds), dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100, || part1(&points, &folds));
    let (res_p2, dur_p2, dur_p2c) = run_many(100, || part2(&points, &folds));

    print_result("P1", res_p1);
    println!("Result (P2):");
    res_p2.print();

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(points: &[(u32, u32)], folds: &[Fold]) -> usize {
    perform_folds(points, folds, Some(1)).len()
}

fn part2(points: &[(u32, u32)], folds: &[Fold]) -> FixedGrid<u8> {
    let points = perform_folds(points, folds, None);
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in points.iter().cloned() {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let mut grid = FixedGrid::new((max_x+1) as usize, (max_y+1) as usize, b' ');
    for (x, y) in points.iter() {
        grid[(*x as usize, *y as usize)] = b'#';
    }

    grid
}

fn perform_folds(points: &[(u32, u32)], mut folds: &[Fold], fold_amount: Option<usize>) -> Vec<(u32, u32)> {
    let mut points = points.to_vec();
    if let Some(fold_amount) = fold_amount {
        folds = &folds[..fold_amount];
    }

    for fold in folds.iter() {
        match fold {
            Fold::X(fold_x) => {
                for (x, _) in points.iter_mut() {
                    if *x > *fold_x {
                        *x -= (*x - *fold_x) * 2;
                    }
                }
            }
            Fold::Y(fold_y) => {
                for (_, y) in points.iter_mut() {
                    if *y > *fold_y {
                        *y -= (*y - *fold_y) * 2;
                    }
                }
            }
        }
    }

    points.sort_unstable();
    points.dedup();

    points
}

enum Fold {
    X(u32),
    Y(u32),
}

fn parse_input(input: &[u8]) -> (Vec<(u32, u32)>, Vec<Fold>) {
    let mut points = Vec::with_capacity(64);
    let mut folds = Vec::with_capacity(32);

    let mut parsing_points = true;
    for line in input.split(|p| *p == b'\n') {
        if line.is_empty() {
            parsing_points = false;
            continue;
        }

        if parsing_points {
            points.push(parse_u32_pair(line));
        } else {
            folds.push(match line[11] {
                b'x' => Fold::X(parse_u32b(&line[13..])),
                b'y' => Fold::Y(parse_u32b(&line[13..])),
                _ => unreachable!(),
            });
        }
    }

    (points, folds)
}
