use smallvec::{SmallVec, smallvec};
use common::aoc::{print_result, run_many, print_time_cold};
use common::parser;
use crate::SnailfishPairPart::{Number, Pair};

fn main() {
    let input_data = include_bytes!("../input/day18.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input_data));
    let (input_sn2, dur_p_2, dur_pc_2) = run_many(1000, || parse_input_sn2(input_data));
    let (res_p1, dur_p1, dur_p1c) = run_many(100, || part1(&input));
    let (res_p1_2, dur_p1_2, dur_p1c_2) = run_many(100, || part1_sn2(&input_sn2));
    let (res_p2, dur_p2, dur_p2c) = run_many(20, || part2(&input));
    let (res_p2_2, dur_p2_2, dur_p2c_2) = run_many(100, || part2_sn2(&input_sn2));

    println!("TREE");

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);

    println!();
    println!("LINEAR");

    print_result("P1", res_p1_2);
    print_result("P2", res_p2_2);

    print_time_cold("Parse", dur_p_2, dur_pc_2);
    print_time_cold("P1", dur_p1_2, dur_p1c_2);
    print_time_cold("P2", dur_p2_2, dur_p2c_2);
    print_time_cold("Total", dur_p_2 + dur_p1_2 + dur_p2_2, dur_pc_2 + dur_p1c_2 + dur_p2c_2);
}

fn part1(input: &[SnailfishNumber]) -> u64 {
    let mut acc = input[0].clone();

    for num in input.iter().skip(1) {
        acc.add(&num);
        acc.reduce();
    }

    acc.magnitude()
}

fn part1_sn2(input: &[SnailfishNumber2]) -> u64 {
    let mut acc = input[0].clone();

    for num in input.iter().skip(1) {
        acc.add(&num);
        acc.reduce();
    }

    acc.magnitude()
}


fn part2(input: &[SnailfishNumber]) -> u64 {
    let mut max_magnitude = 0;

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let mut acc_i = input[i].clone();
            let mut acc_j = input[j].clone();

            acc_i.add(&input[j]);
            acc_j.add(&input[i]);

            acc_i.reduce();
            acc_j.reduce();

            let mag_i = acc_i.magnitude();
            if mag_i > max_magnitude {
                max_magnitude = mag_i;
            }

            let mag_j = acc_j.magnitude();
            if mag_j > max_magnitude {
                max_magnitude = mag_j;
            }
        }
    }

    max_magnitude
}

fn part2_sn2(input: &[SnailfishNumber2]) -> u64 {
    let mut max_magnitude = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            let mut acc = input[i].clone();

            acc.add(&input[j]);
            acc.reduce();

            let magnitude = acc.magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    max_magnitude
}

fn parse_input(input: &[u8]) -> Vec<SnailfishNumber> {
    let mut res = Vec::with_capacity(16);
    let mut input = input;
    while let Some((number, new_input)) = SnailfishNumber::parse(input) {
        res.push(number);
        input = new_input;
    }

    res
}

fn parse_input_sn2(input: &[u8]) -> Vec<SnailfishNumber2> {
    let mut res = Vec::with_capacity(16);
    let mut input = input;
    while let Some((number, new_input)) = SnailfishNumber2::parse(input) {
        res.push(number);
        input = new_input;
    }

    res
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct SnailfishNumber2 {
    parts: SmallVec<[(u64, usize); 32]>,
}

impl SnailfishNumber2 {
    fn explode(&mut self) -> bool {
        for i in 0..self.parts.len() - 1 {
            let (left, l_depth) = self.parts[i];
            let (right, _) = self.parts[i + 1];

            if l_depth == 4 {
                if i > 0 {
                    self.parts[i - 1].0 += left;
                }
                if i < self.parts.len() - 2 {
                    self.parts[i + 2].0 += right;
                }

                self.parts.remove(i + 1);
                self.parts[i] = (0, 3);

                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.parts.len() {
            let (v, depth) = self.parts[i];

            if v >= 10 {
                let left = v / 2;
                let right = left + v % 2;

                self.parts[i] = (left, depth + 1);
                self.parts.insert(i + 1, (right, depth + 1));

                return true;
            }
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, rhs: &SnailfishNumber2) {
        for (_, depth) in self.parts.iter_mut() {
            *depth += 1;
        }
        for (value, depth) in rhs.parts.iter() {
            self.parts.push((*value, *depth + 1));
        }
    }

    fn magnitude(&self) -> u64 {
        let mut sm = self.parts.clone();

        while sm.len() > 1 {
            for i in 0..sm.len() - 1 {
                let (v1, d1) = sm[i];
                let (v2, d2) = sm[i + 1];

                if d1 == d2 {
                    sm.remove(i + 1);
                    sm[i] = ((v1 * 3) + (v2 * 2), d1 - 1);
                    break;
                }
            }
        }

        sm[0].0
    }

    fn parse(mut input: &[u8]) -> Option<(SnailfishNumber2, &[u8])> {
        let mut brackets = 0;
        let mut parts: SmallVec<[(u64, usize); 32]> = SmallVec::new();

        loop {
            if let Some(((), remainder)) = parser::expect_byte(input, b'[') {
                brackets += 1;
                input = remainder;
            } else if let Some(((), remainder)) = parser::expect_byte(input, b']') {
                brackets -= 1;
                input = remainder;
            } else if let Some(((), remainder)) = parser::expect_byte(input, b',') {
                input = remainder;
            } else if let Some((value, remainder)) = parser::uint(input) {
                parts.push((value, brackets - 1));
                input = remainder;
            } else {
                return if parts.len() > 0 {
                    let (_, input) = parser::rest_of_line(input)?;
                    Some((SnailfishNumber2 { parts }, input))
                } else {
                    None
                };
            }
        }
    }
}

struct SnailfishNumberIterator<'a> {
    number: &'a SnailfishNumber,
    stack: SmallVec<[(usize, bool); 16]>,
}

impl<'a> Iterator for SnailfishNumberIterator<'a> {
    type Item = (u64, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, right) = self.stack.pop()?;
        let pair = self.number.pairs[index];

        if right {
            match pair.right {
                Pair(next_index) => {
                    self.stack.push((next_index, true));
                    self.stack.push((next_index, false));
                    self.next()
                }
                Number(number) => {
                    Some((number, index, true))
                }
            }
        } else {
            match pair.left {
                Pair(next_index) => {
                    self.stack.push((next_index, true));
                    self.stack.push((next_index, false));
                    self.next()
                }
                Number(number) => {
                    Some((number, index, false))
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SnailfishNumber {
    pairs: Vec<SnailfishPair>,
    root: usize,
}

impl SnailfishNumber {
    fn numbers(&self) -> SnailfishNumberIterator {
        SnailfishNumberIterator {
            number: self,
            stack: smallvec![(self.root, true), (self.root, false)],
        }
    }

    fn add(&mut self, other: &SnailfishNumber) {
        let offset = self.pairs.len();
        for node in other.pairs.iter() {
            self.pairs.push(node.copy_offset(offset));
        }
        let old_root = self.root;
        self.root = self.pairs.len();
        self.pairs.push(SnailfishPair {
            left: Pair(old_root),
            right: Pair(other.root + offset),
        });
    }

    fn explode_pair(&mut self, parent_index: usize, index: usize) {
        assert!(self.pairs[index].left.is_number());
        assert!(self.pairs[index].right.is_number());

        let mut left_inheritor: Option<(usize, bool)> = None;
        let mut right_inheritor: Option<(usize, bool)> = None;

        let mut pass_mark = (parent_index, true);
        if self.pairs[parent_index].left == Pair(index) {
            self.pairs[parent_index].left = Number(0);
            pass_mark = (parent_index, false);
        } else {
            self.pairs[parent_index].right = Number(0);
        }

        let mut has_passed = false;
        for (_, index, right) in self.numbers() {
            if (index, right) == pass_mark {
                has_passed = true;
                continue;
            }

            if has_passed {
                right_inheritor = Some((index, right));
                break;
            } else {
                left_inheritor = Some((index, right));
            }
        }

        //#[cfg(test)]
        //println!("{:?} {:?}", left_inheritor, right_inheritor);

        if let Some((target, right)) = left_inheritor {
            if right {
                self.pairs[target].right = self.pairs[target].right.add(&self.pairs[index].left);
            } else {
                self.pairs[target].left = self.pairs[target].left.add(&self.pairs[index].left);
            }
        }
        if let Some((target, right)) = right_inheritor {
            if right {
                self.pairs[target].right = self.pairs[target].right.add(&self.pairs[index].right);
            } else {
                self.pairs[target].left = self.pairs[target].left.add(&self.pairs[index].right);
            }
        }
    }

    fn reduce(&mut self) {
        let mut stack: SmallVec<[(usize, usize, usize, bool); 8]> = SmallVec::new();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;

            stack.push((self.root, self.root, 0, true));
            stack.push((self.root, self.root, 0, false));
            while let Some((ci, pi, depth, right)) = stack.pop() {
                let curr = self.pairs[ci];

                if depth >= 4 && curr.left.is_number() && curr.right.is_number() {
                    self.explode_pair(pi, ci);

                    stack.clear();
                    has_changed = true;
                } else {
                    if right {
                        if let Pair(ni) = curr.right {
                            stack.push((ni, ci, depth + 1, true));
                            stack.push((ni, ci, depth + 1, false));
                        }
                    } else {
                        if let Pair(ni) = curr.left {
                            stack.push((ni, ci, depth + 1, true));
                            stack.push((ni, ci, depth + 1, false));
                        }
                    }
                }
            }

            if has_changed {
                continue;
            }

            stack.push((self.root, self.root, 0, true));
            stack.push((self.root, self.root, 0, false));
            while let Some((ci, _, depth, right)) = stack.pop() {
                let curr = self.pairs[ci];

                if right {
                    if let Some((a, b)) = curr.right.must_split() {
                        let new_index = self.pairs.len();
                        self.pairs[ci].right = Pair(new_index);

                        self.pairs.push(SnailfishPair {
                            left: Number(a),
                            right: Number(b),
                        });

                        has_changed = true;
                        stack.clear();
                    } else if let Pair(ni) = curr.right {
                        stack.push((ni, ci, depth + 1, true));
                        stack.push((ni, ci, depth + 1, false));
                    }
                } else {
                    if let Some((a, b)) = curr.left.must_split() {
                        let new_index = self.pairs.len();
                        self.pairs[ci].left = Pair(new_index);

                        self.pairs.push(SnailfishPair {
                            left: Number(a),
                            right: Number(b),
                        });

                        has_changed = true;
                        stack.clear();
                    } else if let Pair(ni) = curr.left {
                        stack.push((ni, ci, depth + 1, true));
                        stack.push((ni, ci, depth + 1, false));
                    }
                }
            }
        }
    }

    fn magnitude_at(&self, pair: usize) -> u64 {
        let pair = &self.pairs[pair];
        let left = match pair.left {
            Number(number) => number,
            Pair(index) => self.magnitude_at(index),
        };
        let right = match pair.right {
            Number(number) => number,
            Pair(index) => self.magnitude_at(index),
        };

        (3 * left) + (2 * right)
    }

    fn magnitude(&self) -> u64 {
        self.magnitude_at(self.root)
    }

    #[allow(dead_code)]
    fn compact_at(&self, new_pairs: &mut Vec<SnailfishPair>, index: usize) -> usize {
        let pair = self.pairs[index];
        let new_pos = new_pairs.len();
        new_pairs.push(SnailfishPair { left: Number(0), right: Number(0) });

        new_pairs[new_pos].left = match pair.left {
            Number(number) => Number(number),
            Pair(old_index) => Pair(self.compact_at(new_pairs, old_index))
        };
        new_pairs[new_pos].right = match pair.right {
            Number(number) => Number(number),
            Pair(old_index) => Pair(self.compact_at(new_pairs, old_index))
        };

        new_pos
    }

    #[allow(dead_code)]
    fn compact(&mut self) {
        let mut new_pairs = Vec::with_capacity(self.pairs.len());

        self.compact_at(&mut new_pairs, self.root);

        self.pairs = new_pairs;
        self.root = 0;
    }

    #[allow(dead_code)]
    fn to_string_at(&self, index: usize) -> String {
        format!("[{},{}]",
                match self.pairs[index].left {
                    Number(v) => v.to_string(),
                    Pair(i) => self.to_string_at(i),
                },
                match self.pairs[index].right {
                    Number(v) => v.to_string(),
                    Pair(i) => self.to_string_at(i),
                },
        )
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.to_string_at(self.root)
    }

    fn parse_pair<'a, 'b>(&'a mut self, input: &'b [u8]) -> Option<(usize, &'b [u8])> {
        let (_, input) = parser::expect_byte(input, b'[')?;

        let pos = self.pairs.len();
        self.pairs.push(SnailfishPair { left: Number(0), right: Number(0) });

        let input = if let Some((v, input)) = parser::uint(input) {
            self.pairs[pos].left = Number(v);

            input
        } else if let Some((pos_2, input)) = self.parse_pair(input) {
            self.pairs[pos].left = Pair(pos_2);

            input
        } else {
            return None;
        };

        let (_, input) = parser::expect_byte(input, b',')?;

        let input = if let Some((v, input)) = parser::uint(input) {
            self.pairs[pos].right = Number(v);

            input
        } else if let Some((pos_2, input)) = self.parse_pair(input) {
            self.pairs[pos].right = Pair(pos_2);

            input
        } else {
            return None;
        };

        let (_, input) = parser::expect_byte(input, b']')?;

        Some((pos, input))
    }

    fn parse(input: &[u8]) -> Option<(SnailfishNumber, &[u8])> {
        let mut number = SnailfishNumber { root: 0, pairs: Vec::with_capacity(16) };

        let (_, input) = number.parse_pair(input)?;
        let (_, input) = parser::rest_of_line(input)?;

        Some((number, input))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SnailfishPair {
    left: SnailfishPairPart,
    right: SnailfishPairPart,
}

impl SnailfishPair {
    fn copy_offset(&self, offset: usize) -> SnailfishPair {
        SnailfishPair {
            left: match self.left {
                Pair(index) => Pair(index + offset),
                Number(number) => Number(number),
            },
            right: match self.right {
                Pair(index) => Pair(index + offset),
                Number(number) => Number(number),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SnailfishPairPart {
    Pair(usize),
    Number(u64),
}

impl SnailfishPairPart {
    fn is_number(&self) -> bool {
        match self {
            Pair(_) => false,
            Number(_) => true,
        }
    }

    fn must_split(&self) -> Option<(u64, u64)> {
        match self {
            Pair(_) => None,
            Number(v) if *v > 9 => Some((
                *v / 2, (*v / 2) + (v % 2)
            )),
            Number(_) => None,
        }
    }

    fn add(&self, other: &SnailfishPairPart) -> SnailfishPairPart {
        match self {
            Number(a) => {
                match other {
                    Number(b) => {
                        Number(*a + *b)
                    }
                    Pair(_) => panic!("add: rhs is SnailfishPairPart::Pair"),
                }
            }
            Pair(_) => panic!("add: lhs is SnailfishPairPart::Pair"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &[u8] = b"[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
";

    const SAMPLE_REDUCE_START: &[u8] = b"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    const SAMPLE_REDUCE_END: &[u8] = b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

    const SAMPLE_MAGNITUDE_1: &[u8] = b"[[1,2],[[3,4],5]]";
    const SAMPLE_MAGNITUDE_2: &[u8] = b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    const SAMPLE_MAGNITUDE_3: &[u8] = b"[[[[1,1],[2,2]],[3,3]],[4,4]]";
    const SAMPLE_MAGNITUDE_4: &[u8] = b"[[[[3,0],[5,3]],[4,4]],[5,5]]";
    const SAMPLE_MAGNITUDE_5: &[u8] = b"[[[[5,0],[7,4]],[5,5]],[6,6]]";
    const SAMPLE_MAGNITUDE_6: &[u8] = b"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
    const SAMPLE_MAGNITUDE_7: &[u8] = b"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";

    const SAMPLE_P1_1: &[u8] = b"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

    const SAMPLE_P1_1_SUM_1: &[u8] = b"[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]";
    const SAMPLE_P1_1_RES_1: &[u8] = b"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";
    const SAMPLE_P1_1_RES_2: &[u8] = b"[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]";
    const SAMPLE_P1_1_RES_3: &[u8] = b"[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]";

    const SAMPLE_P1_2: &[u8] = b"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    const SAMPLE_EXPLODE_1_BEFORE: &[u8] = b"[[[[[9,8],1],2],3],4]";
    const SAMPLE_EXPLODE_1_AFTER: &[u8] = b"[[[[0,9],2],3],4]";
    const SAMPLE_EXPLODE_2_BEFORE: &[u8] = b"[7,[6,[5,[4,[3,2]]]]]";
    const SAMPLE_EXPLODE_2_AFTER: &[u8] = b"[7,[6,[5,[7,0]]]]";
    const SAMPLE_EXPLODE_3_BEFORE: &[u8] = b"[[6,[5,[4,[3,2]]]],1]";
    const SAMPLE_EXPLODE_3_AFTER: &[u8] = b"[[6,[5,[7,0]]],3]";
    const SAMPLE_EXPLODE_4_BEFORE: &[u8] = b"[[[[0,[4,5]],0],0],0]";
    const SAMPLE_EXPLODE_4_AFTER: &[u8] = b"[[[[4,0],5],0],0]";
    const SAMPLE_EXPLODE_5_BEFORE: &[u8] = b"[[0,[0,[0,[1,2]]]],[[0,0],[0,0]]]";
    const SAMPLE_EXPLODE_5_AFTER: &[u8] = b"[[0,[0,[1,0]]],[[2,0],[0,0]]]";

    const SAMPLE_SPLIT_1_BEFORE: &[u8] = b"[[[12,13],12],[13,19]]";
    const SAMPLE_SPLIT_1_AFTER: &[u8] = b"[[[[6,6],[6,7]],[6,6]],[[6,7],[9,[5,5]]]]";

    #[test]
    fn test_parse() {
        let input = SAMPLE_1;
        let (num1, input) = SnailfishNumber::parse(input).unwrap();
        let (num2, input) = SnailfishNumber::parse(input).unwrap();
        let (num3, input) = SnailfishNumber::parse(input).unwrap();
        let (num4, input) = SnailfishNumber::parse(input).unwrap();
        let (num5, input) = SnailfishNumber::parse(input).unwrap();
        let (num6, input) = SnailfishNumber::parse(input).unwrap();
        let (_num7, input) = SnailfishNumber::parse(input).unwrap();
        assert!(SnailfishNumber::parse(input).is_none());

        assert_eq!(num1, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Number(1), right: Number(2) },
            ],
        });

        assert_eq!(num2, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Pair(1), right: Number(3) },
                SnailfishPair { left: Number(1), right: Number(2) },
            ],
        });

        assert_eq!(num3, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Number(9), right: Pair(1) },
                SnailfishPair { left: Number(8), right: Number(7) },
            ],
        });

        assert_eq!(num4, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Pair(1), right: Pair(2) },
                SnailfishPair { left: Number(1), right: Number(9) },
                SnailfishPair { left: Number(8), right: Number(5) },
            ],
        });

        assert_eq!(num5, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Pair(1), right: Number(9) },
                SnailfishPair { left: Pair(2), right: Pair(5) },
                SnailfishPair { left: Pair(3), right: Pair(4) },
                SnailfishPair { left: Number(1), right: Number(2) },
                SnailfishPair { left: Number(3), right: Number(4) },
                SnailfishPair { left: Pair(6), right: Pair(7) },
                SnailfishPair { left: Number(5), right: Number(6) },
                SnailfishPair { left: Number(7), right: Number(8) },
            ],
        });

        assert_eq!(num6, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Pair(1), right: Pair(6) },
                SnailfishPair { left: Pair(2), right: Pair(4) },
                SnailfishPair { left: Number(9), right: Pair(3) },
                SnailfishPair { left: Number(3), right: Number(8) },
                SnailfishPair { left: Pair(5), right: Number(6) },
                SnailfishPair { left: Number(0), right: Number(9) },
                SnailfishPair { left: Pair(7), right: Number(3) },
                SnailfishPair { left: Pair(8), right: Pair(9) },
                SnailfishPair { left: Number(3), right: Number(7) },
                SnailfishPair { left: Number(4), right: Number(9) },
            ],
        });
    }

    #[test]
    fn test_add() {
        let input = SAMPLE_1;
        let (num1, input) = SnailfishNumber::parse(input).unwrap();
        let (num2, input) = SnailfishNumber::parse(input).unwrap();
        let (num3, input) = SnailfishNumber::parse(input).unwrap();
        let (num4, _input) = SnailfishNumber::parse(input).unwrap();

        let mut num1_copy = num1.clone();
        num1_copy.add(&num2);

        let mut num3_copy = num3.clone();
        num3_copy.add(&num4);

        assert_eq!(num1_copy, SnailfishNumber {
            root: 3,
            pairs: vec![
                SnailfishPair { left: Number(1), right: Number(2) },
                SnailfishPair { left: Pair(2), right: Number(3) },
                SnailfishPair { left: Number(1), right: Number(2) },
                SnailfishPair { left: Pair(0), right: Pair(1) },
            ],
        });

        assert_eq!(num3_copy, SnailfishNumber {
            root: 5,
            pairs: vec![
                SnailfishPair { left: Number(9), right: Pair(1) },
                SnailfishPair { left: Number(8), right: Number(7) },
                SnailfishPair { left: Pair(3), right: Pair(4) },
                SnailfishPair { left: Number(1), right: Number(9) },
                SnailfishPair { left: Number(8), right: Number(5) },
                SnailfishPair { left: Pair(0), right: Pair(2) },
            ],
        });
    }

    #[test]
    fn test_compact() {
        let mut test_number = SnailfishNumber {
            root: 5,
            pairs: vec![
                SnailfishPair { left: Number(9), right: Pair(1) },
                SnailfishPair { left: Number(8), right: Number(7) },
                SnailfishPair { left: Pair(3), right: Pair(4) },
                SnailfishPair { left: Number(1), right: Number(9) },
                SnailfishPair { left: Number(8), right: Number(5) },
                SnailfishPair { left: Pair(0), right: Pair(2) },
            ],
        };

        test_number.compact();

        assert_eq!(test_number, SnailfishNumber {
            root: 0,
            pairs: vec![
                SnailfishPair { left: Pair(1), right: Pair(3) },
                SnailfishPair { left: Number(9), right: Pair(2) },
                SnailfishPair { left: Number(8), right: Number(7) },
                SnailfishPair { left: Pair(4), right: Pair(5) },
                SnailfishPair { left: Number(1), right: Number(9) },
                SnailfishPair { left: Number(8), right: Number(5) },
            ],
        })
    }

    #[test]
    fn test_numbers_iterator() {
        let (test1, _) = SnailfishNumber::parse(SAMPLE_P1_1_RES_1).unwrap();
        let (test2, _) = SnailfishNumber::parse(SAMPLE_P1_1_RES_3).unwrap();
        let (test3, _) = SnailfishNumber::parse(SAMPLE_P1_1_SUM_1).unwrap();

        let res1: Vec<u64> = test1.numbers().map(|(v, ..)| v).collect();
        let res2: Vec<u64> = test2.numbers().map(|(v, ..)| v).collect();
        let res3: Vec<u64> = test3.numbers().map(|(v, ..)| v).collect();

        assert_eq!(res1, vec![4u64, 0, 5, 4, 7, 7, 6, 0, 8, 7, 7, 7, 9, 5, 0]);
        assert_eq!(res2, vec![7u64, 0, 7, 7, 7, 7, 7, 8, 7, 7, 8, 8, 7, 7, 8, 7]);
        assert_eq!(res3, vec![0u64, 4, 5, 0, 0, 4, 5, 2, 6, 9, 5, 7, 3, 7, 4, 3, 6, 3, 8, 8]);
    }

    #[test]
    fn test_reduce_explode() {
        let (mut explode1, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_1_BEFORE).unwrap();
        let (explode1_after, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_1_AFTER).unwrap();
        let (mut explode2, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_2_BEFORE).unwrap();
        let (explode2_after, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_2_AFTER).unwrap();
        let (mut explode3, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_3_BEFORE).unwrap();
        let (explode3_after, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_3_AFTER).unwrap();
        let (mut explode4, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_4_BEFORE).unwrap();
        let (explode4_after, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_4_AFTER).unwrap();
        let (mut explode5, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_5_BEFORE).unwrap();
        let (explode5_after, _) = SnailfishNumber::parse(SAMPLE_EXPLODE_5_AFTER).unwrap();

        explode1.reduce();
        explode1.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_EXPLODE_1_BEFORE));
        println!("  actual: {}", explode1.to_string());
        println!("expected: {}", explode1_after.to_string());

        assert_eq!(explode1, explode1_after);

        explode2.reduce();
        explode2.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_EXPLODE_1_BEFORE));
        println!("  actual: {}", explode2.to_string());
        println!("expected: {}", explode2_after.to_string());

        assert_eq!(explode2, explode2_after);

        explode3.reduce();
        explode3.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_EXPLODE_1_BEFORE));
        println!("  actual: {}", explode3.to_string());
        println!("expected: {}", explode3_after.to_string());

        assert_eq!(explode3, explode3_after);

        explode4.reduce();
        explode4.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_EXPLODE_4_BEFORE));
        println!("  actual: {}", explode4.to_string());
        println!("expected: {}", explode4_after.to_string());

        assert_eq!(explode4, explode4_after);

        explode5.reduce();
        explode5.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_EXPLODE_5_BEFORE));
        println!("  actual: {}", explode5.to_string());
        println!("expected: {}", explode5_after.to_string());

        assert_eq!(explode5, explode5_after);
    }

    const SAMPLE_ADD_REDUCE_1_LEFT: &[u8] = b"[[[[4,3],4],4],[7,[[8,4],9]]]";
    const SAMPLE_ADD_REDUCE_1_RIGHT: &[u8] = b"[1,1]";
    const SAMPLE_ADD_REDUCE_1_EXPECTED: &[u8] = b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

    #[test]
    fn test_add_and_reduce_1() {
        let (mut acc, _) = SnailfishNumber::parse(SAMPLE_ADD_REDUCE_1_LEFT).unwrap();
        let (num1, _) = SnailfishNumber::parse(SAMPLE_ADD_REDUCE_1_RIGHT).unwrap();
        let (expected1, _) = SnailfishNumber::parse(SAMPLE_ADD_REDUCE_1_EXPECTED).unwrap();

        println!("  addend: {}", num1.to_string());
        acc.add(&num1);
        println!("   added: {}", acc.to_string());
        acc.reduce();
        acc.compact();
        println!("  actual: {}", acc.to_string());
        println!("expected: {}", expected1.to_string());
        assert_eq!(acc, expected1);
    }

    #[test]
    fn test_add_and_reduce_2() {
        let input = SAMPLE_P1_1;
        let (mut acc, input) = SnailfishNumber::parse(input).unwrap();
        let (num1, input) = SnailfishNumber::parse(input).unwrap();
        let (num2, input) = SnailfishNumber::parse(input).unwrap();
        let (num3, _input) = SnailfishNumber::parse(input).unwrap();

        let (expected1, _) = SnailfishNumber::parse(SAMPLE_P1_1_RES_1).unwrap();
        let (expected2, _) = SnailfishNumber::parse(SAMPLE_P1_1_RES_2).unwrap();
        let (expected3, _) = SnailfishNumber::parse(SAMPLE_P1_1_RES_3).unwrap();

        println!("  addend: {}", num1.to_string());
        acc.add(&num1);
        println!("   added: {}", acc.to_string());
        acc.reduce();
        acc.compact();
        println!("  actual: {}", acc.to_string());
        println!("expected: {}", expected1.to_string());
        assert_eq!(acc, expected1);

        acc.add(&num2);
        acc.reduce();
        acc.compact();
        assert_eq!(acc, expected2);

        acc.add(&num3);
        acc.reduce();
        acc.compact();
        assert_eq!(acc, expected3);
    }

    #[test]
    fn test_reduce_split() {
        let (mut split1, _) = SnailfishNumber::parse(SAMPLE_SPLIT_1_BEFORE).unwrap();
        let (split1_after, _) = SnailfishNumber::parse(SAMPLE_SPLIT_1_AFTER).unwrap();

        split1.reduce();
        split1.compact();
        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_SPLIT_1_BEFORE));
        println!("  actual: {}", split1.to_string());
        println!("expected: {}", split1_after.to_string());
        assert_eq!(split1, split1_after);
    }

    #[test]
    fn test_reduce() {
        let (mut test1, _) = SnailfishNumber::parse(SAMPLE_REDUCE_START).unwrap();
        let (test1_after, _) = SnailfishNumber::parse(SAMPLE_REDUCE_END).unwrap();

        test1.reduce();
        test1.compact();

        println!(" initial: {}", String::from_utf8_lossy(SAMPLE_REDUCE_START));
        println!("  actual: {}", test1.to_string());
        println!("expected: {}", test1_after.to_string());

        assert_eq!(test1, test1_after);
    }

    #[test]
    fn test_magnitude() {
        let (test1, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_1).unwrap();
        let (test2, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_2).unwrap();
        let (test3, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_3).unwrap();
        let (test4, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_4).unwrap();
        let (test5, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_5).unwrap();
        let (test6, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_6).unwrap();
        let (test7, _) = SnailfishNumber::parse(SAMPLE_MAGNITUDE_7).unwrap();

        assert_eq!(test1.magnitude(), 143);
        assert_eq!(test2.magnitude(), 1384);
        assert_eq!(test3.magnitude(), 445);
        assert_eq!(test4.magnitude(), 791);
        assert_eq!(test5.magnitude(), 1137);
        assert_eq!(test6.magnitude(), 3488);
        assert_eq!(test7.magnitude(), 4140);
    }

    #[test]
    fn test_part1() {
        let input1 = parse_input(SAMPLE_P1_1);
        let input2 = parse_input(SAMPLE_P1_2);

        assert_eq!(part1(&input1), 3488);
        assert_eq!(part1(&input2), 4140);
    }

    #[test]
    fn test_part2() {
        let input2 = parse_input(SAMPLE_P1_2);

        assert_eq!(part2(&input2), 3993);
    }

    #[test]
    fn test_sn2_explode() {
        let (mut explode1, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_1_BEFORE).unwrap();
        let (explode1_after, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_1_AFTER).unwrap();
        let (mut explode2, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_2_BEFORE).unwrap();
        let (explode2_after, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_2_AFTER).unwrap();
        let (mut explode3, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_3_BEFORE).unwrap();
        let (explode3_after, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_3_AFTER).unwrap();
        let (mut explode4, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_4_BEFORE).unwrap();
        let (explode4_after, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_4_AFTER).unwrap();
        let (mut explode5, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_5_BEFORE).unwrap();
        let (explode5_after, _) = SnailfishNumber2::parse(SAMPLE_EXPLODE_5_AFTER).unwrap();

        explode1.reduce();
        assert_eq!(explode1, explode1_after);

        explode2.reduce();
        assert_eq!(explode2, explode2_after);

        explode3.reduce();
        assert_eq!(explode3, explode3_after);

        explode4.reduce();
        assert_eq!(explode4, explode4_after);

        explode5.reduce();
        assert_eq!(explode5, explode5_after);
    }

    #[test]
    fn test_sn2_parse() {
        let input = SAMPLE_1;
        let (num1, input) = SnailfishNumber2::parse(input).unwrap();
        let (num2, input) = SnailfishNumber2::parse(input).unwrap();
        let (num3, input) = SnailfishNumber2::parse(input).unwrap();
        let (num4, input) = SnailfishNumber2::parse(input).unwrap();
        let (num5, input) = SnailfishNumber2::parse(input).unwrap();
        let (num6, input) = SnailfishNumber2::parse(input).unwrap();
        let (num7, input) = SnailfishNumber2::parse(input).unwrap();
        assert!(SnailfishNumber::parse(input).is_none());

        assert_eq!(num1, SnailfishNumber2 {
            parts: smallvec![
                (1, 0), (2, 0)
            ],
        });

        assert_eq!(num2, SnailfishNumber2 {
            parts: smallvec![
                (1, 1), (2, 1), (3, 0)
            ],
        });

        assert_eq!(num3, SnailfishNumber2 {
            parts: smallvec![
                (9, 0), (8, 1), (7, 1)
            ],
        });

        assert_eq!(num4, SnailfishNumber2 {
            parts: smallvec![
                (1, 1), (9, 1), (8, 1), (5,1)
            ],
        });

        assert_eq!(num5, SnailfishNumber2 {
            parts: smallvec![
                (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 0)
            ],
        });

        assert_eq!(num6, SnailfishNumber2 {
            parts: smallvec![
                (9, 2), (3, 3), (8, 3), (0, 3), (9, 3), (6,2), (3, 3), (7,3), (4,3), (9,3), (3,1)
            ],
        });

        assert_eq!(num7, SnailfishNumber2 {
            parts: smallvec![
                (1, 3), (3, 3), (5, 3), (3, 3), (1, 3), (3, 3),
                (8, 3), (7, 3), (4, 3), (9, 3), (6, 3), (9, 3),
                (8, 3), (2, 3), (7, 3), (3, 3),
            ],
        });
    }

    #[test]
    fn test_sn2_add() {
        let input = SAMPLE_1;
        let (num1, input) = SnailfishNumber2::parse(input).unwrap();
        let (num2, input) = SnailfishNumber2::parse(input).unwrap();
        let (num3, input) = SnailfishNumber2::parse(input).unwrap();
        let (num4, _input) = SnailfishNumber2::parse(input).unwrap();

        let mut num1_copy = num1.clone();
        num1_copy.add(&num2);

        let mut num3_copy = num3.clone();
        num3_copy.add(&num4);

        assert_eq!(num1_copy, SnailfishNumber2 {
            parts: smallvec![
                (1, 1), (2, 1), (1, 2), (2, 2), (3, 1)
            ],
        });

        assert_eq!(num3_copy, SnailfishNumber2 {
            parts: smallvec![
                (9, 1), (8, 2), (7, 2), (1, 2), (9, 2), (8, 2), (5, 2)
            ],
        });
    }

    #[test]
    fn test_sn2_reduce() {
        let (mut test1, _) = SnailfishNumber2::parse(SAMPLE_REDUCE_START).unwrap();
        let (test1_after, _) = SnailfishNumber2::parse(SAMPLE_REDUCE_END).unwrap();

        test1.reduce();

        assert_eq!(test1, test1_after);
    }

    #[test]
    fn test_sn2_add_and_reduce() {
        let input = SAMPLE_P1_1;
        let (mut acc, input) = SnailfishNumber2::parse(input).unwrap();
        let (num1, input) = SnailfishNumber2::parse(input).unwrap();
        let (num2, input) = SnailfishNumber2::parse(input).unwrap();
        let (num3, _input) = SnailfishNumber2::parse(input).unwrap();

        let (expected1, _) = SnailfishNumber2::parse(SAMPLE_P1_1_RES_1).unwrap();
        let (expected2, _) = SnailfishNumber2::parse(SAMPLE_P1_1_RES_2).unwrap();
        let (expected3, _) = SnailfishNumber2::parse(SAMPLE_P1_1_RES_3).unwrap();

        acc.add(&num1);
        acc.reduce();
        assert_eq!(acc, expected1);

        acc.add(&num2);
        acc.reduce();
        assert_eq!(acc, expected2);

        acc.add(&num3);
        acc.reduce();
        assert_eq!(acc, expected3);
    }

    #[test]
    fn test_sn2_magnitude() {
        let (test1, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_1).unwrap();
        let (test2, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_2).unwrap();
        let (test3, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_3).unwrap();
        let (test4, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_4).unwrap();
        let (test5, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_5).unwrap();
        let (test6, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_6).unwrap();
        let (test7, _) = SnailfishNumber2::parse(SAMPLE_MAGNITUDE_7).unwrap();

        assert_eq!(test1.magnitude(), 143);
        assert_eq!(test2.magnitude(), 1384);
        assert_eq!(test3.magnitude(), 445);
        assert_eq!(test4.magnitude(), 791);
        assert_eq!(test5.magnitude(), 1137);
        assert_eq!(test6.magnitude(), 3488);
        assert_eq!(test7.magnitude(), 4140);
    }

    #[test]
    fn test_sn2_part1() {
        let input1 = parse_input_sn2(SAMPLE_P1_1);
        let input2 = parse_input_sn2(SAMPLE_P1_2);

        assert_eq!(part1_sn2(&input1), 3488);
        assert_eq!(part1_sn2(&input2), 4140);
    }

    #[test]
    fn test_sn2_part2() {
        let input2 = parse_input_sn2(SAMPLE_P1_2);

        assert_eq!(part2_sn2(&input2), 3993);
    }
}

