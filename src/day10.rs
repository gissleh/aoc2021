use common::aoc::{print_result, run_many, print_time_cold};

const SCORES: [u32; 4] = [
    3, 57, 1197, 25137,
];

fn main() {
    let input = include_bytes!("../input/day10.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let ((res_p1, res_p2), dur_p12, dur_p112c) = run_many(1000, || part1(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1+P2", dur_p12, dur_p112c);
    print_time_cold("Total", dur_p + dur_p12, dur_pc + dur_p112c);
}

fn part1(input: &[Bracket]) -> (u32, u64) {
    let mut stack = Vec::with_capacity(64);
    let mut p1_score = 0;
    let mut p2_scores = Vec::with_capacity(32);

    'outer: for line in input.split(|p| *p == Bracket::Newline) {
        if line.is_empty() {
            continue;
        }

        stack.clear();

        for v in line.iter() {
            match *v {
                Bracket::Open(idx) => { stack.push(idx); }
                Bracket::Close(idx) => {
                    if stack.is_empty() || stack.pop().unwrap() != idx {
                        p1_score += SCORES[idx];
                        continue 'outer;
                    }
                }
                _ => {}
            }
        }

        let mut score = 0;
        while let Some(v) = stack.pop() {
            score *= 5;
            score += (v as u64) + 1;
        }

        p2_scores.push(score);
    }

    p2_scores.sort();
    let p2_score = p2_scores[(p2_scores.len() / 2)];

    (p1_score, p2_score)
}

#[derive(Eq, PartialEq)]
enum Bracket {
    Open(usize),
    Close(usize),
    Newline,
}

fn parse_input(input: &[u8]) -> Vec<Bracket> {
    input.iter().map(|i| {
        match *i {
            b'(' => Bracket::Open(0),
            b'[' => Bracket::Open(1),
            b'{' => Bracket::Open(2),
            b'<' => Bracket::Open(3),
            b')' => Bracket::Close(0),
            b']' => Bracket::Close(1),
            b'}' => Bracket::Close(2),
            b'>' => Bracket::Close(3),
            b'\n' => Bracket::Newline,
            _ => unreachable!(),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P2_SAMPLE: &[u8] = b"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_both_parts() {
        let input = parse_input(P2_SAMPLE);

        assert_eq!(part1(&input), (26397, 288957));
    }
}
