use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::{valid_offsets, TinyGrid};

fn main() {
    let input = include_bytes!("../input/day11.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input::<10, 100>(input));
    let ((res_p1, res_p2), dur_p12, dur_p12c) = run_many(1000, || puzzle(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1+P2", dur_p12, dur_p12c);
    print_time_cold("Total", dur_p + dur_p12, dur_pc + dur_p12c);
}

fn puzzle<const W: usize, const S: usize>(input: &TinyGrid<u8, W, S>) -> (usize, u32) {
    let mut stack = Vec::with_capacity(64);
    let mut grid = *input;
    let mut total_flashes = 0;
    let h = S / W;

    for n in 1.. {
        let mut flashes = 0;

        for y in 0..h {
            for x in 0..W {
                let v = grid.get_mut(x, y).unwrap();
                *v += 1;
                if *v > 9 {
                    stack.push((x, y));
                }
            }
        }

        while let Some((x, y)) = stack.pop() {
            grid.set(x, y, 0);
            flashes += 1;

            for (x2, y2) in valid_offsets(true, x, y, W, h) {
                let v = grid.get_mut(x2, y2).unwrap();
                if *v > 0 && *v < 10 {
                    *v += 1;
                    if *v > 9 {
                        stack.push((x2, y2));
                    }
                }
            }
        }

        if n <= 100 {
            total_flashes += flashes;
        }

        if flashes == S {
            return (total_flashes, n);
        }
    }

    unreachable!()
}

fn parse_input<const W: usize, const S: usize>(input: &[u8]) -> TinyGrid<u8, W, S> {
    let mut arr = [0u8; S];
    for (i, v) in input.iter().filter(|v| **v != b'\n').map(|v| *v - b'0').enumerate() {
        arr[i] = v;
    }

    TinyGrid::new(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    const SMALL_SAMPLE: &[u8] = b"11111
19991
19191
19991
11111";

    #[test]
    fn test_both_parts() {
        let input = parse_input::<10, 100>(SAMPLE);

        assert_eq!(puzzle(&input), (1656, 195));
    }

    #[test]
    /// This test is mostly for seeing that the const generic parameters work.
    fn test_both_parts_small() {
        let input = parse_input::<5, 25>(SMALL_SAMPLE);

        assert_eq!(puzzle(&input), (34, 6));
    }
}