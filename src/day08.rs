use common::aoc::{print_result, run_many, print_time_cold};

const SEPARATOR: u8 = 0b10000000;
const NEWLINE: u8 = 0b00000000;

fn main() {
    let input = include_bytes!("../input/day08.txt");

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

fn part1(input: &[u8]) -> u32 {
    let mut count = 0;
    let mut active = false;

    for v in input.iter() {
        if *v == NEWLINE {
            active = false;
        } else if *v == SEPARATOR {
            active = true;
        }

        if active {
            let ones = v.count_ones();
            if ones == 2 || ones == 3 || ones == 4 || ones == 7 {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &[u8]) -> u32 {
    let mut sum = 0;
    let number_start = 1 + input.iter().enumerate().find(|(_, v)| **v == SEPARATOR).map(|(i, _)| i).unwrap();

    for line in input.split(|b| *b == NEWLINE) {
        if line.is_empty() {
            continue;
        }

        let one = *line.iter().find(|v| v.count_ones() == 2).unwrap();
        let four = *line.iter().find(|v| v.count_ones() == 4).unwrap();

        let mut number = 0;
        for n in line[number_start..].iter() {
            number *= 10;
            number += match n.count_ones() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 if *n & one == one => 3,
                5 if (*n & four).count_ones() == 3 => 5,
                5 => 2,
                6 if *n & four == four => 9,
                6 if *n & one == one => 0,
                6 => 6,
                7 => 8,
                _ => unreachable!(),
            }
        }

        sum += number;
    }

    sum
}

fn parse_input(input: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(64);
    let mut current = 0u8;
    let mut active = false;

    for ch in input.iter() {
        match ch {
            b'a'..=b'g' => {
                current |= 1 << (ch - b'a') as usize;
                active = true;
            }
            b'|' => {
                res.push(SEPARATOR);
            }
            _ => {
                if active {
                    res.push(current);
                    current = 0;
                    active = false;

                    if *ch == b'\n' {
                        res.push(NEWLINE);
                    }
                }
            }
        }
    }

    if active {
        res.push(current);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE: &[u8] = b"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n";
    const FULL_TEST_CASE: &[u8] = b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1_full() {
        let parsed = parse_input(FULL_TEST_CASE);

        assert_eq!(part1(&parsed), 26);
    }

    #[test]
    fn test_part2() {
        let parsed = parse_input(TEST_CASE);

        assert_eq!(part2(&parsed), 5353);
    }

    #[test]
    fn test_part2_full() {
        let parsed = parse_input(FULL_TEST_CASE);

        assert_eq!(part2(&parsed), 61229);
    }
}
