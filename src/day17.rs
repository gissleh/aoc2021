use common::aoc::{print_result, run_many, print_time_cold};
use common::parsers::{parse_i32s_amount};

fn main() {
    let input = include_bytes!("../input/day17.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(1, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);

    assert!(res_p2 > 188);
}

fn part1(target: &TargetArea) -> i32 {
    let mut highest_y = 0;
    for sy in 1..-target.br_y {
        let mut found = None;

        'loop_sx: for sx in 1..target.br_x {
            let mut vx = sx;
            let mut vy = sy;
            let mut x = 0;
            let mut y = 0;
            let mut y_peak = 0;

            loop {
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;

                if y > y_peak {
                    y_peak = y;
                }

                match target.check(x, y) {
                    TargetStatus::Hit => {
                        if let Some(other_y_peak) = found {
                            if y_peak > other_y_peak {
                                found = Some(y_peak)
                            }
                        } else {
                            found = Some(y_peak)
                        }
                        break 'loop_sx;
                    }
                    TargetStatus::Overshot => {
                        break 'loop_sx;
                    }
                    TargetStatus::Undershot => {
                        break;
                    }
                    TargetStatus::Undetermined => {
                        // Do nothing.
                    }
                }
            }
        }

        if let Some(peak_y) = found {
            highest_y = peak_y;
        }
    }

    highest_y
}

fn part2(target: &TargetArea) -> usize {
    let mut count = 0;

    for sy in target.br_y..-target.br_y {
        for sx in 1..=target.br_x {
            let mut vx = sx;
            let mut vy = sy;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;

                match target.check(x, y) {
                    TargetStatus::Hit => {
                        count += 1;
                        break;
                    }
                    TargetStatus::Overshot | TargetStatus::Undershot => {
                        break;
                    }
                    TargetStatus::Undetermined => {}
                }
            }
        }
    }

    count
}

fn parse_input(input: &[u8]) -> TargetArea {
    let mut arr = [0i32; 4];
    parse_i32s_amount(input, &mut arr, 4);

    TargetArea::from(&arr)
}

#[derive(Eq, PartialEq, Debug)]
enum TargetStatus {
    Undershot,
    Overshot,
    Undetermined,
    Hit,
}

struct TargetArea {
    tl_x: i32,
    tl_y: i32,
    br_x: i32,
    br_y: i32,
}

impl From<&[i32; 4]> for TargetArea {
    fn from(arr: &[i32; 4]) -> Self {
        Self {
            tl_x: arr[0],
            tl_y: arr[3],
            br_x: arr[1],
            br_y: arr[2],
        }
    }
}

impl TargetArea {
    #[inline]
    fn check(&self, x: i32, y: i32) -> TargetStatus {
        if y < self.br_y {
            if x < self.tl_x {
                TargetStatus::Undershot
            } else {
                TargetStatus::Overshot
            }
        } else if y > self.tl_y {
            if x > self.br_x {
                TargetStatus::Overshot
            } else {
                TargetStatus::Undetermined
            }
        } else {
            if x >= self.tl_x && x <= self.br_x {
                TargetStatus::Hit
            } else if x > self.br_x {
                TargetStatus::Overshot
            } else if x < self.tl_x && y > self.br_y {
                TargetStatus::Undetermined
            } else {
                TargetStatus::Undershot
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [i32; 4] = [20, 30, -10, -5];
    const SAMPLE_INPUT_2: [i32; 4] = [352, 377, -49, -30];

    #[test]
    fn test_target_area() {
        let ta = TargetArea::from(&SAMPLE_INPUT);

        assert_eq!(ta.check(0, 0), TargetStatus::Undetermined);
        assert_eq!(ta.check(21, 3), TargetStatus::Undetermined);
        assert_eq!(ta.check(21, -5), TargetStatus::Hit);
        assert_eq!(ta.check(20, -5), TargetStatus::Hit);
        assert_eq!(ta.check(33, -9), TargetStatus::Overshot);
        assert_eq!(ta.check(17, -5), TargetStatus::Undetermined);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TargetArea::from(&SAMPLE_INPUT)), 45);
        assert_eq!(part1(&TargetArea::from(&SAMPLE_INPUT_2)), 66);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TargetArea::from(&SAMPLE_INPUT)), 112);
        assert_eq!(part2(&TargetArea::from(&SAMPLE_INPUT_2)), 820);
    }
}