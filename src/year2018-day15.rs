use std::thread::sleep;
use std::time::Duration;
use time::PreciseTime;
use common::aoc::{print_result, run_many, print_time_cold, run_many_mut, run_once, print_time, run_once_mut};
use common::grid::{BFS, BFSStep, FixedGrid};
use common::parsers::{parse_u32_list};
use crate::Piece::Empty;

const OFFSETS: [(usize, usize); 4] = [
    (0, !0),
    (!0, 0),
    (1, 0),
    (0, 1),
];

fn main() {
    let input = include_bytes!("../input/year2018-day15.txt");

    let (mut board, dur_p, dur_pc) = run_many(1000, || Board::parse(input));
    let (res_p2, dur_p2) = run_once_mut(|| part2(&mut board));
    print_result("P2", res_p2);
    let (res_p1, dur_p1) = run_once_mut(|| part1(&mut board));
    print_result("P1", res_p1);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time("P1", dur_p1);
    print_time("P2", dur_p2);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1 + dur_p2);
}

fn part1(board: &mut Board) -> u32 {
    board.reset();

    for n in 0.. {
        if board.run_turn(3) {
            return n * board.total_hp();
        }
    }

    0
}

fn part2(board: &mut Board) -> u32 {
    for a in 4.. {
        board.reset();

        println!("initial_state");
        board.print();

        for n in 0.. {
            let done = board.run_turn(a);

            let full_rounds = if done { n } else { n + 1 };
            println!("full_rounds={} total_hp={} elfpower={} elves={} goblins={}", full_rounds, board.total_hp() ,a, board.remaining_elves, board.remaining_goblins);
            board.print();

            if board.elf_died {
                break;
            }

            if done {
                return n * board.total_hp();
            }
        }
    }

    0
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Wall,
    Empty,
    Player(u8, u8, u32),
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Wall
    }
}

#[derive(Clone)]
struct Board {
    initial_grid: FixedGrid<Piece>,
    initial_elves: u32,
    initial_goblins: u32,
    grid: FixedGrid<Piece>,
    bfs: BFS<(usize, usize)>,
    elf_died: bool,
    remaining_elves: u32,
    remaining_goblins: u32,
}

impl Board {
    fn reset(&mut self) {
        self.grid.copy_from(&self.initial_grid);
        self.elf_died = false;
        self.remaining_elves = self.initial_elves;
        self.remaining_goblins = self.initial_goblins;
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut players: Vec<(char, u32)> = Vec::with_capacity(64);

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                match self.grid.get(x, y).unwrap() {
                    Piece::Empty => print!("."),
                    Piece::Wall => print!("#"),
                    Piece::Player(team, _, hp) => {
                        let ch = if *team == b'G' {'G'} else {'E'};
                        players.push((ch, *hp));

                        print!("{}", ch);
                    }
                }
            }

            for (ch, hp) in players.iter() {
                print!(" {}({})", *ch, *hp);
            }
            players.clear();

            println!();
        }

        println!();
    }

    fn total_hp(&self) -> u32 {
        let mut total = 0;
        for (_, _, piece) in self.grid.iter() {
            if let Piece::Player(_, _, hp) = *piece {
               total += hp
            }
        }

        total
    }

    fn run_turn(&mut self, elf_attack_power: u32) -> bool {
        let mut has_moved = [false; 255];

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                if let Some(Piece::Player(team, idx, _)) = self.grid.get(x, y).cloned() {
                    if has_moved[idx as usize] {
                        continue;
                    }
                    has_moved[idx as usize] = true;

                    let other_team = if team == b'E' { b'G' } else { b'E' };
                    let attack_power = if team == b'E' { elf_attack_power } else { 3 };
                    let has_target = if team == b'E' { self.remaining_goblins > 0 } else { self.remaining_elves > 0 };
                    if !has_target {
                        return true;
                    }

                    // Pre-attack
                    if self.try_attacking(x, y, other_team, attack_power) {
                        continue;
                    }

                    // Make a move and attack if the path was short
                    if let Some((new_x, new_y)) = self.move_player(x, y, other_team) {
                        self.try_attacking(new_x, new_y, other_team, attack_power);
                    }
                }
            }
        }

        false
    }

    fn try_attacking(&mut self, x: usize, y: usize, other_team: u8, attack_power: u32) -> bool {
        if let Some((x2, y2, Piece::Player(_, idx, hp2))) = self.find_neighbor(x, y, other_team) {
            if hp2 > attack_power {
                self.grid.set(x2, y2, Piece::Player(other_team, idx, hp2 - attack_power))
            } else {
                if other_team == b'E' {
                    self.elf_died = true;
                    self.remaining_elves -= 1;
                } else {
                    self.remaining_goblins -= 1;
                }

                self.grid.set(x2, y2, Piece::Empty);
            }

            true
        } else {
            false
        }
    }

    fn move_player(&mut self, x: usize, y: usize, team: u8) -> Option<(usize, usize)> {
        let res = self.bfs.run(&self.grid, x, y, false, |piece, (x2, y2), s| {
            if (x2, y2) == (x, y) {
                return BFSStep::Continue(*s)
            }

            let res = match piece {
                Piece::Wall => BFSStep::DeadEnd,
                Piece::Player(t, _, _) if *t == team => BFSStep::Found(*s),
                Piece::Player(_, _, _) => BFSStep::DeadEnd,
                Piece::Empty if *s == (0, 0) => BFSStep::Continue((x2, y2)),
                Piece::Empty => BFSStep::Continue(*s),
            };

            res
        });

        if let Some((_, l, (x2, y2))) = res {
            let player = *self.grid.get(x, y).unwrap();
            self.grid.set(x2, y2, player);
            self.grid.set(x, y, Piece::Empty);

            Some((x2, y2))
        } else {
            None
        }
    }

    fn find_neighbor(&mut self, x: usize, y: usize, team: u8) -> Option<(usize, usize, Piece)> {
        let mut result = None;

        for (x_offset, y_offset) in OFFSETS.iter() {
            let x2 = x.wrapping_add(*x_offset);
            if x2 >= self.grid.width() {
                continue;
            }
            let y2 = y.wrapping_add(*y_offset);
            if y2 >= self.grid.height() {
                continue;
            }

            if let Some(piece) = self.grid.get(x2, y2) {
                if let Piece::Player(piece_team, _, hp) = piece {
                    if *piece_team == team {
                        if let Some((_, _, Piece::Player(_, _, hp2))) = result {
                            if *hp < hp2 {
                                result = Some((x2, y2, *piece));
                            }
                        } else {
                            result = Some((x2, y2, *piece));
                        }
                    }
                }
            }
        }

        result
    }

    fn parse(input: &[u8]) -> Board {
        let width = input.iter().take_while(|v| **v != b'\n').count();
        let mut next = 0;
        let data: Vec<Piece> = input.iter().filter(|v| **v != b'\n').map(|i| {
            match *i {
                b'.' => Piece::Empty,
                b'#' => Piece::Wall,
                b'E' | b'G' => {
                    let idx = next;
                    next += 1;
                    Piece::Player(*i, idx, 200)
                },
                _ => unreachable!(),
            }
        }).collect();
        let height = data.len() / width;

        let mut elves = 0;
        let mut goblins = 0;
        for piece in data.iter() {
            if let Piece::Player(team, _, _) = piece {
                if *team == b'E' {
                    elves += 1;
                } else {
                    goblins += 1;
                }
            }
        }

        Board {
            initial_grid: FixedGrid::from(width, height, data),
            grid: FixedGrid::blank(width, height),
            bfs: BFS::new(),
            elf_died: false,
            initial_goblins: goblins,
            initial_elves: elves,
            remaining_elves: 0,
            remaining_goblins: 0,
        }
    }
}

fn parse_input(input: &[u8]) -> Vec<u32> {
    parse_u32_list(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &[u8] = b"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const SAMPLE2: &[u8] = b"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    const SAMPLE3: &[u8] = b"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const SAMPLE4: &[u8] = b"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const SAMPLE5: &[u8] = b"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const MOVEMENT_SAMPLE: &[u8] = b"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";

    #[test]
    fn test_part1() {
        let mut board = Board::parse(SAMPLE1);

        assert_eq!(part1(&mut board), 27730);
    }


    #[test]
    fn test_part2() {
        let mut board = Board::parse(SAMPLE1);
        let mut board2 = Board::parse(SAMPLE2);
        let mut board3 = Board::parse(SAMPLE3);
        let mut board4 = Board::parse(SAMPLE4);
        let mut board5 = Board::parse(SAMPLE5);
        let mut board6 = Board::parse(MOVEMENT_SAMPLE);

        assert_eq!(part2(&mut board), 4988);
        assert_eq!(part2(&mut board2), 1140);
        assert_eq!(part2(&mut board3), 31284);
        assert_eq!(part2(&mut board4), 6474);
        assert_eq!(part2(&mut board5), 3478);
        assert_eq!(part2(&mut board6), 1328);
    }
}