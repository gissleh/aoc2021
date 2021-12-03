use num::pow;
use common::aoc::{load_input, print_result, print_time, run_many, run_once, print_time_cold};
use common::parsers::{parse_binary_u32};

fn main() {
    let (input, dur_load) = run_once(|| load_input("day03"));

    print_time("Load", dur_load);

    let ((input, bits), dur_p, dur_pc) = run_many(1000, || parse_input(&input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10000, || part1(&input, bits));
    let (res_p2, dur_p2, dur_p2c) = run_many(10000, || part2(&input, bits));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[u32], bits: usize) -> u32 {
    let mut acc = [0usize; 31];
    for curr in input.iter() {
        let mut bit = 1;
        for i in 0..bits {
            if *curr & bit != 0 {
                acc[i] += 1;
            }

            bit *= 2;
        }
    }

    let mut res = 0;
    let mut bit = 1;
    let threshold = input.len() / 2;
    for count in acc.iter().take(bits) {
        if *count > threshold {
            res |= bit;
        }
        bit *= 2;
    }

    let mask = pow(2, bits) - 1;

    res * (!res & (mask))
}

fn part2(input: &[u32], bits: usize) -> u32 {
    check_rating(Vec::from(input), bits, false)
        * check_rating(Vec::from(input), bits, true)
}

fn check_rating(mut input: Vec<u32>, bits: usize, least: bool) -> u32 {
    let mut bit = pow(2, bits-1);

    while input.len() > 1 {
        let count_ones = input.iter()
            .filter(|v| **v & bit > 0)
            .count();

        let threshold = (input.len()+1) / 2;
        let ones_least = count_ones < threshold;
        let keep_ones = ones_least == least;

        let mut i = 0;
        while i < input.len() {
            if (input[i] & bit > 0) != keep_ones {
                input.swap_remove(i);
            } else {
                i += 1;
            }
        }

        bit /= 2;
    }

    input[0]
}

fn parse_input(input: &str) -> (Vec<u32>, usize) {
    (
        input.lines().map(parse_binary_u32).collect(),
        input.lines().next().unwrap().len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [u32; 12] = [
        0b00100,
        0b11110,
        0b10110,
        0b10111,
        0b10101,
        0b01111,
        0b00111,
        0b11100,
        0b10000,
        0b11001,
        0b00010,
        0b01010,
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&TEST_DATA, 5), 198)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&TEST_DATA, 5), 230)
    }
}