use common::aoc::{load_input, print_result, print_time, run_many, run_once, print_time_cold};
use common::parsers::parse_u32;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day01"));

    print_time("Load", dur_load);

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(&input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(100000, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[u32]) -> usize {
    // diff =
    // + A A A 0
    // - 0 B B B
    // = A    -B
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(parse_u32)
        .collect()
}