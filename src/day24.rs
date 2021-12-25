use smallvec::SmallVec;
use common::aoc::{print_result, run_many, print_time_cold};

const A_LIST: [i64; 14] = [1,1,1,26,1,26,1,1,26,1,26,26,26,26];
const B_LIST: [i64; 14] = [12,10,13,-11,13,-1,10,11,0,10,-5,-16,-7,-11];
const C_LIST: [i64; 14] = [6,6,3,11,9,3,13,6,14,10,12,10,11,15];

fn main() {
    let (res_p1, dur_p1, dur_p1c) = run_many(1000, || part1());
    let (res_p2, dur_p2, dur_p2c) = run_many(1000, || part2());

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p1 + dur_p2, dur_p1c + dur_p2c);
}

fn part1() -> i64 {
    puzzle([9; 14], false)
}

fn part2() -> i64 {
    puzzle([1; 14], true)
}

fn puzzle(initial: [i64; 14], asc: bool) -> i64 {
    let mut digits = initial;

    while let Some((index, prev_index, diff)) = run_digits(&digits) {
        digits[index] = digits[prev_index] + diff;

        let (lowest, highest) = if digits[index] >= digits[prev_index] {
            (digits[prev_index], digits[index])
        } else {
            (digits[index], digits[prev_index])
        };

        if asc {
            let diff = lowest - 1;
            digits[index] -= diff;
            digits[prev_index] -= diff;
        } else {
            let diff = 9 - highest;
            digits[index] += diff;
            digits[prev_index] += diff;
        }

        assert_eq!(digits[index], digits[prev_index] + diff);
    }

    let mut z = 0;
    for i in 0..14 {
        z = run_step(z, digits[i], i);
    }
    assert_eq!(z, 0);

    digits.iter().fold(0, |p, c| p * 10 + *c)
}

fn run_digits(digits: &[i64]) -> Option<(usize, usize, i64)> {
    let mut stack: SmallVec<[(usize, i64); 16]> = SmallVec::new();
    let divs = A_LIST;
    let checks = B_LIST;
    let offsets = C_LIST;

    for i in 0..digits.len() {
        if divs[i] == 1 {
            stack.push((i, digits[i] + offsets[i]));
        } else {
            let (popped_index, popped_v) = stack.pop().unwrap();
            let check_value = popped_v + checks[i];
            let diff = check_value - digits[i];

            if check_value != digits[i] {
                return Some((i, popped_index, diff));
            }
        }
    }

    None
}

#[cfg(test)]
fn run_step_slow(z: i64, w: i64, idx: usize) -> i64 {
    let a = A_LIST[idx]; // 26
    let b = B_LIST[idx]; // 10
    let c = C_LIST[idx]; // 14

    let mut z = z; // 1
    // inp w (1)

    // mul x 0 (1)
    let mut x = 0; // 0
    // add x z (1)
    x += z; //
    // mod x 26 (1)
    x %= 26;
    // div z 1 (0)
    z /= a;
    // add x 12 (12)
    x += b;
    // eql x w (0)
    x = if x == w { 1 } else { 0 };
    // eql x 0 (1)
    x = if x == 0 { 1 } else { 0 };
    // mul y 0 (0)
    let mut y = 0;
    // add y 25 (25)
    y += 25;
    // mul y x (25)
    y *= x;
    // add y 1 (26)
    y += 1;
    // mul z y (0)
    z *= y;
    // mul y 0 (0)
    y *= 0;
    // add y w (1)
    y += w;
    // add y 6 (21)
    y += c;
    // mul y x (21)
    y *= x;
    // add z y (21)
    z += y;

    z
}

fn run_step(z: i64, w: i64, idx: usize) -> i64 {
    let a = A_LIST[idx];
    let b = B_LIST[idx];
    let c = C_LIST[idx];

    let mut z = z;

    // div z 1 (or 26)
    let condition = ((z % 26) + b) != w; // stack.peek() + b != w
    z /= a; // stack.pop();

    // mul x 0
    // add x z
    // mod x 26
    // add x 12 (or 13,10,0,-5,-7,-11...)
    // eql x w
    // eql x 0
    if condition {
        // mul y 0
        // add y 25
        // mul y x
        // add y 1
        // mul z y
        // mul y 0
        // add y w
        // add y 6 (or 3..=14)
        // mul y x
        // add z y
        z = (z * 26) + (w + c);
    }

    z
}

#[cfg(test)]
mod tests {
    use crate::{reverse_step, run_step, run_step_slow};

    #[test]
    fn can_reverse() {
        for idx in 0..12 {
            for z in -77..77 {
                for w in 1..9 {
                    println!("{},{},{}", idx, z, w);

                    assert_eq!(
                        run_step(
                            run_step(
                                run_step(z, w, idx),
                                w, idx + 1),
                            w, idx + 2,
                        ) % 26,
                        run_step(
                            run_step(z, w, idx + 1),
                            w, idx + 2,
                        ) % 26,
                    )
                }
            }
        }
    }

    #[test]
    fn optimization_is_correct() {
        for idx in 0..14 {
            for z in -77..77 {
                for w in 1..9 {
                    assert_eq!(
                        run_step(z, w, idx),
                        run_step_slow(z, w, idx),
                    )
                }
            }
        }
    }
}
