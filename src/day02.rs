use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::parse_u32b;

fn main() {
    let input = include_bytes!("../input/day02.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(100000, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[(i32, i32)]) -> i32 {
    let (horizontal_sum, depth_sum) = input.iter()
        .cloned()
        .fold((0,0),|(px, py),(cx,cy)|(px+cx,py+cy));

    horizontal_sum * depth_sum
}

fn part2(input: &[(i32, i32)]) -> i32 {
    let (horizontal_sum, depth_sum) = input.iter()
        .cloned()
        .scan(0, |aim, (x, y)| {
            *aim += y;
            Some((x, x * *aim))
        })
        .fold((0,0), |(px, py),(cx,cy)|(px+cx,py+cy));

    horizontal_sum * depth_sum
}

fn parse_input(input: &[u8]) -> Vec<(i32, i32)> {
    input.split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|v| {
            let dir = v[0];
            let space_pos = v.iter().take_while(|b| **b != b' ').count();
            let len = parse_u32b(&v[space_pos+1..]) as i32;

            match dir {
                b'd' => (0, len),
                b'u' => (0, -len),
                b'f' => (len, 0),
                _ => unreachable!(),
            }
        })
        .collect()
}
