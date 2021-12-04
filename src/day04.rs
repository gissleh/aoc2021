use common::aoc::{print_result, print_time, run_many, run_once, print_time_cold, load_input_bytes};
use common::parsers::{parse_u32s_amount, parse_u32s_until};

const ROW_MASK: u32 = 0b1111100000000000000000000;
const COL_MASK: u32 = 0b1000010000100001000010000;

fn main() {
    let (input, dur_load) = run_once(|| load_input_bytes("day04"));

    print_time("Load", dur_load);

    let (input, dur_p, dur_pc) = run_many(1000, || Bingo::parse(&input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &Bingo) -> u32 {
    let mut marks = vec![0u32; input.boards.len()];

    for n in input.numbers.iter() {
        let index = *n as usize;

        for (i, board) in input.boards.iter().enumerate() {
            let board_index = board.indices[index];
            let mark = marks.get_mut(i).unwrap();

            let found = if board_index > 0 {
                *mark |= 1 << (board_index - 1);
                true
            } else {
                false
            };

            if found && mark.count_ones() >= 5 {
                for j in 0..5u32 {
                    let row = ROW_MASK >> (j *5);
                    let col = COL_MASK >> (j *1);

                    if *mark & row == row {
                        return board.score(*mark, *n);
                    }
                    if *mark & col == col {
                        return board.score(*mark, *n);
                    }
                }
            }
        }
    }

    0
}

fn part2(input: &Bingo) -> u32 {
    let mut marks = vec![0u32; input.boards.len()];
    let mut won = vec![false; input.boards.len()];
    let mut wins_left = input.boards.len();

    for n in input.numbers.iter() {
        let index = *n as usize;

        for (i, board) in input.boards.iter().enumerate() {
            if won[i] {
                continue;
            }

            let board_index = board.indices[index];
            let mark = marks.get_mut(i).unwrap();

            let found = if board_index > 0 {
                *mark |= 1 << (board_index - 1);
                true
            } else {
                false
            };

            if found && mark.count_ones() >= 5 {
                for j in 0..5u32 {
                    let row = ROW_MASK >> (j *5);
                    let col = COL_MASK >> (j *1);

                    if *mark & row == row {
                        won[i] = true;
                        wins_left -= 1;
                        break
                    } else if *mark & col == col {
                        won[i] = true;
                        wins_left -= 1;
                        break
                    }
                }

                if wins_left == 0 {
                    return board.score(*mark, *n);
                }
            }
        }
    }

    0}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

struct Board {
    numbers: Vec<u32>,
    indices: Vec<usize>,
}

impl Board {
    pub fn score(&self, marks: u32, just_called: u32) -> u32 {
        let mut unmarked = 0;
        let mut bit = 1;

        for n in self.numbers.iter() {
            if marks & bit == 0 {
                unmarked += *n
            }

            bit *= 2;
        }

        unmarked * just_called
    }
}

impl Bingo {
    pub fn parse(input: &[u8]) -> Bingo {
        let mut numbers = Vec::with_capacity(128);
        let mut boards = Vec::with_capacity(16);

        let mut pos = parse_u32s_until(input, &mut numbers, b'\n');
        while pos < input.len() - 2 {
            let mut curr_board = Board{
                numbers: Vec::with_capacity(25),
                indices: vec![0; numbers.len() + 1],
            };


            pos += parse_u32s_amount(&input[pos..], &mut curr_board.numbers, 25);
            for (i, n) in curr_board.numbers.iter().enumerate() {
                curr_board.indices[*n as usize] = i+1;
            }

            boards.push(curr_board);
        }

        Bingo { numbers, boards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let mut bingo = Bingo::parse(SAMPLE_INPUT.as_bytes());

        assert_eq!(part1(&bingo), 4512);
    }

    #[test]
    fn test_part2() {
        let mut bingo = Bingo::parse(SAMPLE_INPUT.as_bytes());

        assert_eq!(part2(&bingo), 1924);
    }
}