use common::aoc::{load_input, print_result, print_time, run_many, run_once};

fn main() {
    let (input, dur_load) = run_once(|| load_input("dayXX"));

    print_time("Load", dur_load);

    let (OBJNAME, dur_parse) = run_many(1000, || parse_input(&input));
    //let (res_part1, dur_part1) = run_many(100000, || part1(&OBJNAME));
    //let (res_part2, dur_part2) = run_many(100000, || part2(&OBJNAME));

    //print_result("P1", res_part1);
    //print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    //print_time("P1", dur_part1);
    //print_time("P2", dur_part2);
    //print_time("Total", dur_parse + dur_part1 + dur_part2);
}

fn parse_input(input: &str) -> OBJTYPE {

}