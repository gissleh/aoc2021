use common::aoc::{load_input, print_result, print_time, run_many, run_once, print_time_cold};

fn main() {
    let (input, dur_load) = run_once(|| load_input("day02"));

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

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|v| {
            let (dir, len_str) = v.split_once(' ').unwrap();
            let len = len_str.parse::<i32>().unwrap();

            match dir {
                "down" => (0, len),
                "up" => (0, -len),
                "forward" => (len, 0),
                _ => unreachable!(),
            }
        })
        .collect()
}
