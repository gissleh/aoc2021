use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_u32_list};

fn main() {
    let input = include_bytes!("../input/dayXX.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[u32]) -> u32 {
    0
}

fn part2(input: &[u32]) -> u32 {
    0
}

fn parse_input(input: &[u8]) -> Vec<u32> {
    parse_u32_list(input)
}
