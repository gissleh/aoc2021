use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::{FixedGrid, Dijkstra, DijkstraStep};

fn main() {
    let input = include_bytes!("../input/day15.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(50, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn checker<'a>(target: &'a (usize, usize)) -> impl Fn(&'a i64, (usize, usize)) -> DijkstraStep {
    |v, pos| {
        if pos == *target {
            DijkstraStep::Found(*v)
        } else {
            DijkstraStep::Continue(*v, 0)
        }
    }
}

fn part1(input: &FixedGrid<i64>) -> i64 {
    let mut dijkstra = Dijkstra::new(false, 0, 0, 0);
    let target = (input.width()-1, input.height()-1);
    dijkstra.run(input, checker(&target));

    dijkstra.found_cost().unwrap()
}

fn part2(input: &FixedGrid<i64>) -> i64 {
    let mut big_grid = FixedGrid::new(input.width() * 5, input.height() * 5, 0);
    for y in 0..big_grid.height() {
        let sy = y % input.height();
        let my = (y / input.height()) as i64;

        for x in 0..big_grid.width() {
            let sx = x % input.width();
            let mx = (x / input.width()) as i64;

            let mut v = input[(sx, sy)] + (mx + my);
            if v >= 10 {
                v -= 9;
            }

            big_grid.set(x, y, v);
        }
    }

    let mut dijkstra = Dijkstra::new(false, 0, 0, 0);
    let target = (big_grid.width()-1, big_grid.height()-1);
    dijkstra.run(&big_grid, checker(&target));

    dijkstra.found_cost().unwrap()
}

fn parse_input(input: &[u8]) -> FixedGrid<i64> {
    let width = input.iter().take_while(|v| **v != b'\n').count();
    let data = input.iter()
        .filter(|v| **v != b'\n')
        .map(|v| (*v - b'0') as i64)
        .collect::<Vec<i64>>();
    let height = data.len() / width;

    FixedGrid::from(width, height, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &[u8] = b"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part2(&input), 315);
    }
}