use common::aoc::{print_result, run_many, print_time_cold};
use num::Integer;

fn main() {
    let input = include_bytes!("../input/day14.txt");

    let ((initial, polymerization), dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || puzzle(&initial, &polymerization, 10));
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || puzzle(&initial, &polymerization, 40));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);

    assert_ne!(res_p2, 3447389044529);
}

fn puzzle(input: &[usize], rules: &[Option<usize>; 100], count: usize) -> u64 {
    let mut vector = [0u64; 100];
    for v in input.windows(2) {
        let v = v[0] * 10 + v[1];
        vector[v] += 1;
    }

    for _ in 0..count {
        let mut vector2 = [0u64; 100];

        for (i, c) in rules.iter().enumerate() {
            if let Some(c) = c {
                let a = i / 10;
                let b = i % 10;
                let ac = (a * 10) + *c;
                let cb = (*c * 10) + b;

                vector2[ac] += vector[i];
                vector2[cb] += vector[i];
            } else {
                vector2[i] += vector[i];
            }
        }

        vector = vector2;
    }

    let mut res = [0u64; 10];
    for v in input.iter() {
        res[*v] += 1;
    }
    for (i, count) in vector.iter().enumerate() {
        res[i / 10] += *count;
        res[i % 10] += *count;
    }

    let min = res.iter().filter(|v| **v > 0).min().unwrap();
    let max = res.iter().max().unwrap();
    let diff = *max - *min;

    if diff.is_odd() {
        (diff / 2) + 1
    } else {
        diff / 2
    }
}

fn parse_input(input: &[u8]) -> (Vec<usize>, [Option<usize>; 100]) {
    let letters_exist = input.iter()
        .filter(|v| **v >= b'A' && **v <= b'Z')
        .fold([false; 26], |mut f, i| {
            f[(i - b'A') as usize] = true;
            f
        });
    let mut polymerization = [None; 100];

    let mut index_map = [0usize; 26];
    let mut next_index = 0usize;
    for i in 0..26 {
        if letters_exist[i] {
            index_map[i] = next_index;
            next_index += 1;
        }
    }
    assert!(next_index <= 10);

    let initial_state = input.iter()
        .take_while(|v| **v != b'\n')
        .map(|v| index_map[(*v - b'A') as usize])
        .collect();

    for line in input.split(|v| *v == b'\n').skip(2) {
        if line.is_empty() {
            continue;
        }

        let a = index_map[(line[0] - b'A') as usize];
        let b = index_map[(line[1] - b'A') as usize];
        let c = index_map[(line[6] - b'A') as usize];

        polymerization[(a * 10) + b] = Some(c);
    }

    (
        initial_state,
        polymerization,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &[u8] = b"NNCB

HB -> C
HH -> N
CB -> H
CH -> B
NH -> C
HC -> B
HN -> C
NN -> C
CC -> N
NC -> B
BH -> H
BB -> N
NB -> B
BN -> B
BC -> B
CN -> C
";

    #[test]
    fn test_part1() {
        let (input, polymerization) = parse_input(SAMPLE_1);

        assert_eq!(puzzle(&input, &polymerization, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let (input, polymerization) = parse_input(SAMPLE_1);

        assert_eq!(puzzle(&input, &polymerization, 40), 2188189693529);
    }
}
