use common::aoc::{load_input, print_result, print_time, run_many, run_once, print_time_cold};
use common::parsers::{parse_u32};
use common::grid::FixedGrid;

fn main() {
    let (input, dur_load) = run_once(|| load_input("year2019-day18"));

    print_time("Load", dur_load);

    let (input, dur_p, dur_pc) = run_many(1000, || FixedGrid::parse_str(&input));
    //let (res_p1, dur_p1, dur_p1c) = run_many(100000, || part1(&input));
    //let (res_p2, dur_p2, dur_p2c) = run_many(100000, || part2(&input));

    //print_result("P1", res_p1);
    //print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    //print_time_cold("P1", dur_p1, dur_p1c);
    //print_time_cold("P2", dur_p2, dur_p2c);
    //print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn dfs(grid: &FixedGrid<char>, x: usize, y: usize, target: char) -> (u32, Vec<char>) {
    let mut list = Vec::with_capacity(16);
    let mut distance = 0;

    

    (distance, list)
}