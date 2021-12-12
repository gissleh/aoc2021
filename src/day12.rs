use smallvec::{SmallVec, smallvec};
use common::aoc::{print_result, run_many, print_time_cold};

fn main() {
    let input = include_str!("../input/day12.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || Map::parse(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(100, || input.count_paths(false));
    let (res_p2, dur_p2, dur_p2c) = run_many(100, || input.count_paths(true));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}


struct Map<'a> {
    start_index: usize,
    caves: Vec<Cave<'a>>,
}

struct Cave<'a> {
    name: &'a str,
    index: usize,
    kind: CaveKind,
    exits: SmallVec<[usize; 8]>,
}

enum CaveKind {
    Start,
    End,
    Big,
    Small,
}

struct Search {
    index: usize,
    single: bool,
    path: SmallVec<[usize; 16]>,
    mask: u64,
}

impl Search {
    fn next(&self, index: usize, single: bool) -> Search {
        let mut path = self.path.clone();
        path.push(index);

        Search{
            path, index,
            single: single || self.single,
            mask: self.mask | (1 << index),
        }
    }
}

impl<'a> Cave<'a> {
    fn parse(name: &'a str) -> Cave<'a> {
        Cave {
            name,
            exits: SmallVec::new(),
            kind: match name {
                "start" => CaveKind::Start,
                "end" => CaveKind::End,
                _ => match name.chars().next().unwrap() {
                    'a'..='z' => CaveKind::Small,
                    'A'..='Z' => CaveKind::Big,
                    _ => unreachable!(),
                }
            },
            index: 0,
        }
    }
}

impl<'a> Map<'a> {
    fn count_paths(&self, single_twice: bool) -> usize {
        let mut count = 0;
        let mut queue = Vec::with_capacity(64);
        queue.push(Search {
            index: self.start_index,
            mask: 1 << self.start_index,
            path: smallvec![self.start_index],
            single: false,
        });

        while let Some(current) = queue.pop() {
            let cave = &self.caves[current.index];

            for exit_index in cave.exits.iter() {
                let exit = &self.caves[*exit_index];
                match exit.kind {
                    CaveKind::Start => {}
                    CaveKind::Big => {
                        queue.push(current.next(*exit_index, false));
                    }
                    CaveKind::End => {
                        count += 1;
                    }
                    CaveKind::Small => {
                        if (1 << exit_index) & current.mask == 0 {
                            queue.push(current.next(*exit_index, false));
                        } else if single_twice && !current.single {
                            queue.push(current.next(*exit_index, true));
                        }
                    }
                }
            }
        }

        count
    }

    fn parse(input: &'a str) -> Map<'a> {
        let mut caves: Vec<Cave> = Vec::with_capacity(32);
        let mut start_index = 0;

        for line in input.lines() {
            if let Some((left, right)) = line.split_once('-') {
                let mut left_index = !0;
                let mut right_index = !0;

                for (i, cave) in caves.iter().enumerate() {
                    if cave.name == left {
                        left_index = i;
                    }
                    if cave.name == right {
                        right_index = i;
                    }
                }

                if left_index == !0 {
                    left_index = caves.len();
                    caves.push(Cave::parse(left));
                    caves[left_index].index = left_index;
                }
                if right_index == !0 {
                    right_index = caves.len();
                    caves.push(Cave::parse(right));
                    caves[right_index].index = right_index;
                }

                caves[left_index].exits.push(right_index);
                caves[right_index].exits.push(left_index);

                if start_index == 0 && left == "start" {
                    start_index = left_index;
                }
            }
        }

        Map { caves, start_index }
    }
}
