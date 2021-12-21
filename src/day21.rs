use common::aoc::{print_result, run_many, print_time_cold};
use common::parser;
use std::cmp::max;

fn main() {
    let input = include_bytes!("../input/day21.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(50, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &[u32; 2]) -> u32 {
    let mut dice = 0;
    let mut scores = [0u32; 2];
    let mut positions = *input;

    for turn in 0.. {
        let i = turn % 2;
        let current_position = &mut positions[i];
        let current_score = &mut scores[i];

        let mut roll = 0;
        for _ in 0..3 {
            dice += 1;
            if dice == 101 {
                dice = 1;
            }

            roll += dice;
        }

        *current_position = (*current_position + roll) % 10;
        *current_score += *current_position + 1;

        if *current_score >= 1000 {
            let total_rolls = ((turn as u32) + 1) * 3;
            let losing_score = scores[(i+1)%2];
            return losing_score * total_rolls;
        }
    }

    0
}

fn part2(input: &[u32; 2]) -> u64 {
    let mut cache: Vec<Option<(u64, u64)>> = vec![None; 2097152];

    let (p1_wins, p2_wins) = calculate_universes(*input, [0, 0], 0, &mut cache);
    max(p1_wins, p2_wins)
}

const WEIGHTS: [u64; 10] = [
    0, 0, 0, 1, 3,
    6, 7, 6, 3, 1,
];

fn calculate_universes(positions: [u32; 2], scores: [u32; 2], turn: u32, cache: &mut [Option<(u64, u64)>]) -> (u64, u64) {
    let key = cache_key(&positions, &scores, turn);

    if let Some(v) = cache[key] {
        v
    } else {
        let mut sum = (0, 0);
        let ti = turn as usize;
        let next_turn = (turn + 1) % 2;

        for dice in 3..10 {
            let weight = WEIGHTS[dice as usize];
            let mut new_scores = scores;
            let mut new_positions = positions;

            new_positions[ti] = (new_positions[ti] + dice) % 10;
            new_scores[ti] += new_positions[ti] + 1;
            if new_scores[ti] >= 21 {
                if turn == 0 {
                    sum.0 += weight;
                } else {
                    sum.1 += weight;
                }
            } else {
                let sum2 = calculate_universes(new_positions, new_scores, next_turn, cache);
                sum.0 += sum2.0 * weight;
                sum.1 += sum2.1 * weight;
            }
        }

        cache[key] = Some(sum);
        sum
    }
}

fn cache_key(positions: &[u32; 2], scores: &[u32; 2], turn: u32) -> usize {
    // Positions 0..9 (4 bits)
    // Scores: 0..23 (5 bits)
    // Turn: 0..1 (1 bit)
    ((positions[0] << 17) | (positions[1] << 13) | (scores[0] << 8) | (scores[1] << 3) | turn) as usize
}

fn parse_input(input: &[u8]) -> [u32; 2] {
    let ((ai, asi), input) = parse_player(input).unwrap();
    let ((bi, bsi), _) = parse_player(input).unwrap();

    let mut res = [0u32; 2];
    res[ai] = asi;
    res[bi] = bsi;

    res
}

fn parse_player(input: &[u8]) -> Option<((usize, u32), &[u8])> {
    let (_, input) = parser::expect_bytes(input, b"Player ")?;
    let (player_number, input) = parser::uint::<usize>(input)?;
    let (_, input) = parser::expect_bytes(input, b" starting position: ")?;
    let (starting_position, input) = parser::uint::<u32>(input)?;
    let (_, input) = parser::rest_of_line(input)?;

    Some(((player_number - 1, starting_position - 1), input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[3, 7]), 739785);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[3, 7]), 444356092776315u64);
    }
}