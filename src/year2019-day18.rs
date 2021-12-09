use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::{FixedGrid, BFS, BFSStep};
use rustc_hash::FxHashMap;

const ALPHA: &[u8] = b"@abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input = include_str!("../input/year2019-day18.txt");

    let (input, dur_p, dur_pc) = run_many(10, || FixedGrid::<u8>::from_str(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10, || part1(&input));
    let (input, dur_m, dur_mc) = run_many(100, || modify_input(&input));
    let (res_p2, dur_p2, dur_p2c) = run_many(10, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("Modify", dur_m, dur_mc);
    print_time_cold("P2", dur_p2, dur_p2c);
    //print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(maze: &FixedGrid<u8>) -> usize {
    let tree = MazeTree::build(maze);
    let mut stack: Vec<(usize, usize, u32, usize)> = Vec::with_capacity(32);
    let target_mask = (1 << tree.keys.len()) - 1;
    let mut shortest = 0;
    let mut shortest_to = FxHashMap::default();
    stack.push((0, 0, 1, 0));

    while let Some((key_index, path_index, key_mask, len)) = stack.pop() {
        let key = &tree.keys[key_index];

        if key_mask == target_mask {
            if shortest == 0 || len < shortest {
                shortest = len;
            }
        }

        for i in path_index..key.paths.len() {
            let path = &key.paths[i];
            let new_len = len + path.len;

            if shortest > 0 && len + path.len > shortest {
                continue;
            }
            if key_mask & (1 << path.to) != 0 {
                continue;
            }

            let new_mask = key_mask | (1 << (path.to));

            let st_key = (key_mask as usize) * 100 + path.to;
            if let Some(v) = shortest_to.get_mut(&st_key) {
                if new_len < *v {
                    *v = new_len;
                } else {
                    continue;
                }
            } else {
                shortest_to.insert(st_key, new_len);
            }

            if path.doors & !key_mask == 0 {
                stack.push((key_index, i + 1, key_mask, len));
                stack.push((path.to, 0, new_mask, len + path.len));
                break;
            }
        }
    }

    shortest
}

fn modify_input(maze: &FixedGrid<u8>) -> FixedGrid<u8> {
    let (sx, sy) = maze.find(b'@').unwrap();
    let mut maze2 = maze.clone();

    maze2.set(sx - 1, sy - 1, b'@');
    maze2.set(sx, sy - 1, b'#');
    maze2.set(sx + 1, sy - 1, b'@');
    maze2.set(sx - 1, sy, b'#');
    maze2.set(sx, sy, b'#');
    maze2.set(sx + 1, sy, b'#');
    maze2.set(sx - 1, sy + 1, b'@');
    maze2.set(sx, sy + 1, b'#');
    maze2.set(sx + 1, sy + 1, b'@');

    maze2
}

fn part2(maze: &FixedGrid<u8>) -> usize {
    let tree = MazeTree::build(maze);
    let mut stack: Vec<([usize; 4], usize, usize, u32, usize)> = Vec::with_capacity(32);
    let target_mask = (1 << tree.keys.len()) - 1;
    let mut shortest = 0;
    let mut shortest_to = FxHashMap::default();
    stack.push(([0, 0, 0, 0], 0, 0, 1, 0));

    while let Some((robots, robot_index, path_index, key_mask, len)) = stack.pop() {
        if key_mask == target_mask {
            if shortest == 0 || len < shortest {
                shortest = len;
            }
        }

        for i in robot_index..4 {
            let key_index = robots[i];
            let key = &tree.keys[key_index];

            for j in path_index..key.paths.len() {
                let path = &key.paths[j];
                let new_len = len + path.len;

                if shortest > 0 && len + path.len > shortest {
                    continue;
                }
                if key_mask & (1 << path.to) != 0 {
                    continue;
                }

                let new_mask = key_mask | (1 << (path.to));

                let st_key = (key_mask as usize) * 100 + path.to;
                if let Some(v) = shortest_to.get_mut(&st_key) {
                    if new_len < *v {
                        *v = new_len;
                    } else {
                        continue;
                    }
                } else {
                    shortest_to.insert(st_key, new_len);
                }

                let mut new_robots = robots;
                new_robots[i] = path.to;

                if path.doors & !key_mask == 0 {
                    stack.push((robots, i, j + 1, key_mask, len));
                    stack.push((new_robots, 0, 0, new_mask, new_len));
                    break;
                }
            }
        }
    }

    shortest
}

fn search(maze: &FixedGrid<u8>, from_x: usize, from_y: usize, target: u8) -> (usize, u32, (usize, usize)) {
    let mut bfs = BFS::<u32>::new();

    if let Some((_, len, door_mask)) = bfs.run(&maze, from_x, from_y, false, |curr, door_mask| {
        if *curr == target {
            BFSStep::Found(*door_mask)
        } else {
            match *curr {
                b'A'..=b'Z' => {
                    let new_door_mask = *door_mask | (1 << ((*curr - b'A') + 1) as u32);
                    BFSStep::Continue(new_door_mask)
                }
                b'a'..=b'z' => {
                    BFSStep::Continue(*door_mask)
                }
                b'.' | b'@' => {
                    BFSStep::Continue(*door_mask)
                }
                _ => {
                    BFSStep::DeadEnd
                }
            }
        }
    }) {
        (len, door_mask, bfs.found_pos().unwrap())
    } else {
        (0, 0, (0, 0))
    }
}

struct MazeTree {
    keys: Vec<MTKey>,
}

struct MTKey {
    x: usize,
    y: usize,
    quadrant: usize,
    paths: Vec<MTPath>,
}

struct MTPath {
    to: usize,
    len: usize,
    doors: u32,
}

impl MazeTree {
    fn build(maze: &FixedGrid<u8>) -> MazeTree {
        let mut keys = Vec::new();

        for i in 0..27 {
            if let Some((x, y)) = maze.find(ALPHA[i]) {
                keys.push(MTKey {
                    x,
                    y,
                    quadrant: 0,
                    paths: Vec::with_capacity(32),
                })
            } else {
                break;
            }
        }

        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                let (dist, doors, (x, y)) = search(&maze, keys[j].x, keys[j].y, ALPHA[i]);
                if dist == 0 {
                    continue;
                }

                if i == 0 {
                    let bit_1 = if x == keys[0].x { 0 } else { 1 };
                    let bit_2 = if y == keys[0].y { 0 } else { 2 };
                    keys[j].quadrant = bit_1 | bit_2;
                }

                //println!("{} -> {} ({} {:#032b}) ({})", ALPHA[i] as char, ALPHA[j] as char, dist, doors, keys[j].quadrant);

                keys[i].paths.push(MTPath {
                    doors,
                    to: j,
                    len: dist,
                });
                keys[j].paths.push(MTPath {
                    doors,
                    to: i,
                    len: dist,
                });
            }
        }

        MazeTree { keys }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1A: &str = "#########
#b.A.@.a#
#########";

    const SAMPLE_1B: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const SAMPLE_1C: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const SAMPLE_1D: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const SAMPLE_1E: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    const SAMPLE_2A: &str = "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######";

    const SAMPLE_2B: &str = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";

    const SAMPLE_2C: &str = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";

    #[test]
    fn test_search() {
        let maze = FixedGrid::<u8>::from_str(SAMPLE_1A);

        let (sb_len, sb_doors, _) = search(&maze, 5, 1, b'b');
        let (sa_len, sa_doors, _) = search(&maze, 5, 1, b'a');

        assert_eq!(sb_doors, 2);
        assert_eq!(sb_len, 4);
        assert_eq!(sa_doors, 0);
        assert_eq!(sa_len, 2);
    }

    #[test]
    fn test_part1() {
        let maze1 = FixedGrid::<u8>::from_str(SAMPLE_1A);
        let maze2 = FixedGrid::<u8>::from_str(SAMPLE_1B);
        let maze3 = FixedGrid::<u8>::from_str(SAMPLE_1C);
        let maze4 = FixedGrid::<u8>::from_str(SAMPLE_1D);
        let maze5 = FixedGrid::<u8>::from_str(SAMPLE_1E);

        assert_eq!(part1(&maze1), 8);
        assert_eq!(part1(&maze2), 86);
        assert_eq!(part1(&maze3), 132);
        assert_eq!(part1(&maze4), 136);
        assert_eq!(part1(&maze5), 81);
    }

    #[test]
    fn test_part2() {
        let maze1 = FixedGrid::<u8>::from_str(SAMPLE_2A);
        let maze2 = FixedGrid::<u8>::from_str(SAMPLE_2B);
        let maze3 = FixedGrid::<u8>::from_str(SAMPLE_2C);

        assert_eq!(part2(&maze1), 8);
        assert_eq!(part2(&maze2), 24);
        assert_eq!(part2(&maze3), 32);
    }
}