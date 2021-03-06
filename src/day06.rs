use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_u32_list};
use common::matrix::matrix_times_vector;

const MATRIX: [[u64; 9]; 9] = [
    [0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
];

fn main() {
    let input = include_bytes!("../input/day06.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100000, || puzzle(&input, 80));
    let (res_p2, dur_p2, dur_p2c) = run_many(100000, || puzzle(&input, 256));
    let (res_p2m, dur_p2m, dur_p2mc) = run_many(100000, || puzzle_m(&input, 256));

    print_result("P1", res_p1);
    print_result("P2 (Shift)", res_p2);
    print_result("P2 (Matrix)", res_p2m);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2 (Shift)", dur_p2, dur_p2c);
    print_time_cold("P2 (Matrix)", dur_p2m, dur_p2mc);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn puzzle(input: &[u32], amount: usize) -> u64 {
    let mut hist = [0u64; 9];

    for v in input.iter() {
        hist[*v as usize] += 1;
    }

    for _ in 0..amount {
        let mut next_hist = [0u64; 9];

        for i in 1..9 {
            next_hist[i - 1] += hist[i];
        }
        next_hist[6] += hist[0];
        next_hist[8] += hist[0];

        hist = next_hist;
    }

    hist.iter().sum()
}

fn puzzle_m(input: &[u32], amount: usize) -> u64 {
    let mut vector = [0u64; 9];
    for v in input.iter() {
        vector[*v as usize] += 1;
    }

    for _ in 0..amount {
        vector = matrix_times_vector(MATRIX, vector);
    }

    vector.iter().sum()
}

fn parse_input(input: &[u8]) -> Vec<u32> {
    parse_u32_list(input)
}
