use std::cmp::Ordering;
use std::collections::BinaryHeap;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use common::aoc::{print_result, run_many, print_time_cold};

const HALLWAY_EXITS: [usize; 4] = [2, 4, 6, 8];
const HALLWAY_ALLOWED: [bool; 11] = [true, true, false, true, false, true, false, true, false, true, true];
const COST_MULTIPLIER: [usize; 4] = [1, 10, 100, 1000];

fn main() {
    let input = include_bytes!("../input/day23.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || parse_input(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10, || part1(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(10, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1<const N: usize>(input: &GameState<N>) -> usize {
    let mut visited: FxHashMap<GameState<N>, usize> = FxHashMap::default();
    let mut heap: BinaryHeap<GameStateCost<N>> = BinaryHeap::with_capacity(128);
    heap.push(GameStateCost(*input, 0));
    visited.insert(*input, 0);

    while let Some(GameStateCost(state, cost)) = heap.pop() {
        // Winner, winner, chicken dinner?
        if state.is_winner() {
            return cost;
        }

        // Can anyone go home today?
        for hallway_index in 0..state.hallway.len() {
            let piece = state.hallway[hallway_index];
            if piece == 0 {
                continue;
            }
            let room_index = (piece - 1) as usize;

            if let Some(slot_index) = state.can_move_home(hallway_index) {
                let cost_multiplier = COST_MULTIPLIER[room_index];
                let new_cost = cost + (cost_multiplier * move_cost(room_index, slot_index, hallway_index));

                let mut new_state = state;
                new_state.hallway[hallway_index] = 0;
                new_state.rooms[room_index][slot_index] = piece;

                let entry = visited.get(&new_state);
                if entry.is_none() || *entry.unwrap() > new_cost {
                    visited.insert(new_state, new_cost);
                    heap.push(GameStateCost(new_state, new_cost));
                }
            }
        }

        // Anyone ready to leave?
        for room_index in 0..4 {
            let room_piece = (room_index + 1) as u8;
            if state.rooms[room_index].iter().find(|p| **p != room_piece).is_none() {
                continue;
            }

            for slot_index in 0..N {
                let piece = state.rooms[room_index][slot_index];
                if piece == 0 {
                    continue;
                }

                if let Some(hallway_indices) = state.can_move_out(room_index, slot_index) {
                    let cost_multiplier = COST_MULTIPLIER[piece as usize - 1];

                    for hallway_index in hallway_indices {
                        let new_cost = cost + (cost_multiplier * move_cost(room_index, slot_index, hallway_index));

                        let mut new_state = state;
                        new_state.rooms[room_index][slot_index] = 0;
                        new_state.hallway[hallway_index] = piece;

                        let entry = visited.get(&new_state);
                        if entry.is_none() || *entry.unwrap() > new_cost {
                            visited.insert(new_state, new_cost);
                            heap.push(GameStateCost(new_state, new_cost));
                        }
                    }
                }

                // The next boi can't move anyway.
                break;
            }
        }
    }

    0
}

fn part2(input: &GameState<2>) -> usize {
    let unfolded_state = GameState {
        hallway: input.hallway,
        rooms: [
            [input.rooms[0][0], 4, 4, input.rooms[0][1]],
            [input.rooms[1][0], 3, 2, input.rooms[1][1]],
            [input.rooms[2][0], 2, 1, input.rooms[2][1]],
            [input.rooms[3][0], 1, 3, input.rooms[3][1]],
        ],
    };

    part1(&unfolded_state)
}

fn parse_input(input: &[u8]) -> GameState<2> {
    let mut state = GameState::default();
    for (i, v) in input.iter().filter(|v| **v >= b'A' && **v <= b'D').enumerate() {
        state.rooms[i % 4][i / 4] = (*v - b'A') + 1;
    }

    state
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct GameState<const N: usize> {
    hallway: [u8; 11],
    rooms: [[u8; N]; 4],
}

impl<const N: usize> Default for GameState<N> {
    fn default() -> Self {
        GameState {
            hallway: [0; 11],
            rooms: [[0; N]; 4],
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
struct GameStateCost<const N: usize>(GameState<N>, usize);

impl<const N: usize> PartialOrd<Self> for GameStateCost<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for GameStateCost<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
            .then_with(|| self.0.hallway.cmp(&other.0.hallway))
            .then_with(|| self.0.rooms.cmp(&other.0.rooms))
    }
}

impl<const N: usize> GameState<N> {
    fn can_move_home(&self, hallway_index: usize) -> Option<usize> {
        let piece = self.hallway[hallway_index];
        if piece == 0 {
            return None;
        }

        let target_room = (piece - 1) as usize;
        let mut target_slot = 0;
        for i in 0..N {
            if self.rooms[target_room][i] == 0 {
                target_slot = i;
            } else if self.rooms[target_room][i] != piece {
                return None;
            }
        }

        let exit_index = HALLWAY_EXITS[target_room];
        if hallway_index < exit_index {
            for i in hallway_index + 1..=exit_index {
                if self.hallway[i] != 0 {
                    return None;
                }
            }
        } else {
            let mut i = hallway_index - 1;
            loop {
                if self.hallway[i] != 0 {
                    return None;
                }

                if i == exit_index {
                    break;
                }
                i -= 1;
            }
        }

        Some(target_slot)
    }

    fn can_move_out(&self, room_index: usize, slot_index: usize) -> Option<SmallVec<[usize; 8]>> {
        if slot_index > 0 {
            for i in 0..slot_index {
                if self.rooms[room_index][i] > 0 {
                    return None;
                }
            }
        }

        let exit_index = HALLWAY_EXITS[room_index];
        let mut found = SmallVec::new();
        for i in exit_index..self.hallway.len() {
            if HALLWAY_ALLOWED[i] {
                if self.hallway[i] == 0 {
                    found.push(i);
                } else {
                    break;
                }
            }
        }
        let mut i = exit_index;
        loop {
            if HALLWAY_ALLOWED[i] {
                if self.hallway[i] == 0 {
                    found.push(i);
                } else {
                    break;
                }
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }

        if found.len() > 0 {
            Some(found)
        } else {
            None
        }
    }

    fn is_winner(&self) -> bool {
        for i in 0..4 {
            let t = i as u8 + 1;
            for j in 0..N {
                if self.rooms[i][j] != t {
                    return false;
                }
            }
        }

        true
    }
}

fn move_cost(room_index: usize, slot_index: usize, hallway_index: usize) -> usize {
    let exit_index = HALLWAY_EXITS[room_index];
    let hallway_cost = if hallway_index >= exit_index {
        hallway_index - exit_index
    } else {
        exit_index - hallway_index
    };

    hallway_cost + slot_index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_on_example() {
        let input = parse_input(EXAMPLE_P1);
        assert_eq!(part1(&input), 12521);
    }

    #[test]
    fn part2_works_on_example() {
        let input = parse_input(EXAMPLE_P1);
        assert_eq!(part2(&input), 44169);
    }

    #[test]
    fn move_costs_makes_sense() {
        assert_eq!(move_cost(0, 0, 0), 3);
        assert_eq!(move_cost(0, 1, 0), 4);
        assert_eq!(move_cost(1, 1, 0), 6);
        assert_eq!(move_cost(1, 0, 5), 2);
        assert_eq!(move_cost(1, 1, 5), 3);
        assert_eq!(move_cost(1, 1, 2), 4);

        assert_eq!(move_cost(2, 0, 3), 4);
        assert_eq!(move_cost(1, 0, 5), 2);
        assert_eq!(move_cost(2, 0, 5), 2);
        assert_eq!(move_cost(1, 1, 5), 3);
        assert_eq!(move_cost(1, 1, 5), 3);
        assert_eq!(move_cost(1, 1, 3), 3);
        assert_eq!(move_cost(1, 1, 3), 3);
        assert_eq!(move_cost(3, 1, 7), 3);
        assert_eq!(move_cost(3, 1, 5), 5);
    }

    #[test]
    fn amphipods_block_the_way_home() {
        // #############
        // #.....D.D.A.#
        // ###.#B#C#.###
        //   #A#B#C#.#
        //   #########
        let state = GameState {
            hallway: [0, 0, 0, 0, 0, 4, 0, 4, 0, 1, 0],
            rooms: [[0, 1], [2, 2], [3, 3], [0, 0]],
        };

        assert_eq!(state.can_move_home(5), None);
        assert_eq!(state.can_move_home(7), Some(1));
        assert_eq!(state.can_move_home(9), None);
    }

    #[test]
    fn amphipods_block_the_way_out() {
        // #############
        // #.....D.D.A.#
        // ###.#B#C#.###
        //   #A#B#C#.#
        //   #########
        let state = GameState {
            hallway: [0, 0, 0, 0, 0, 4, 0, 4, 0, 1, 0],
            rooms: [[0, 1], [2, 2], [3, 3], [0, 0]],
        };

        // I know Cs aren't supposed to move out, but it's a good test case
        assert_eq!(state.can_move_out(2, 1), None);
        assert_eq!(state.can_move_out(2, 0), None);
    }

    const EXAMPLE_P1: &[u8] = b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
}