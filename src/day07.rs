use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_u32_list};

fn main() {
    let input = include_bytes!("../input/day07.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(100, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn puzzle(input: &[u32], fuel_cb: impl Fn(u32) -> u32) -> u32 {
    let max = *input.iter().max().unwrap();
    let mut winning_fuel = u32::MAX;

    for i in 0..max {
        let mut fuel = 0;
        for crab in input.iter().cloned() {
            let diff = if crab > i {
                crab - i
            } else {
                i - crab
            };

            fuel += fuel_cb(diff);
            if fuel > winning_fuel {
                break
            }
        }

        if fuel < winning_fuel {
            winning_fuel = fuel;
        }
    }

    winning_fuel
}

fn part1(input: &[u32]) -> u32 {
    puzzle(input, |diff| diff)
}

fn part2(input: &[u32]) -> u32 {
    puzzle(input, |diff| (diff * (diff + 1)) / 2)
}

fn parse_input(input: &[u8]) -> Vec<u32> {
    parse_u32_list(input)
}
