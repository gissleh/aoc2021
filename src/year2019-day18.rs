use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::FixedGrid;

const DIRS: [(usize, usize); 4] = [
    (!0, 0),
    (0, !0),
    (1, 0),
    (0, 1),
];

const RET_DIRS: [usize; 4] = [2, 3, 0, 1];

const ALPHA: &[u8] = b"@abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input = include_str!("../input/year2019-day18.txt");

    let (input, dur_p, dur_pc) = run_many(10, || FixedGrid::<u8>::from_str(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(10, || part1(&input));
    //let (res_p2, dur_p2, dur_p2c) = run_many(1000, || puzzle(&lines, width, height, true));

    print_result("P1", res_p1);
    //print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    //print_time_cold("P2", dur_p2, dur_p2c);
    //print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(maze: &FixedGrid<u8>) -> usize {
    let tree = MazeTree::build(maze);
    let mut stack: Vec<(usize, usize, u32, usize)> = Vec::with_capacity(32);
    let target_mask = (1 << tree.keys.len()) -1;
    let mut shortest = 0;
    let mut shortest_to = vec![0; (1<<27)*100];
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
            let new_len = len+path.len;

            if shortest > 0 && len+path.len > shortest {
                continue;
            }
            if key_mask & (1 << path.to) != 0 {
                continue;
            }

            let new_mask = key_mask|(1<<(path.to));

            let st_key = (key_mask as usize) * 100 + path.to;
            if shortest_to[st_key] == 0 || new_len < shortest_to[st_key] {
                shortest_to[st_key] = new_len;
            } else {
                continue;
            }

            if path.doors & !key_mask == 0 {
                stack.push((key_index, i+1, key_mask, len));
                stack.push((path.to, 0, new_mask, len+path.len));
                break;
            }
        }
    }

    shortest
}

fn dfs(maze: &FixedGrid<u8>, from_x: usize, from_y: usize, target: u8) -> (usize, u32) {
    let mut stack: Vec<(usize, usize, usize, usize, usize, u32)> = Vec::with_capacity(200);
    let mut has_searched = FixedGrid::new(maze.width(), maze.height(), 0);
    let mut shortest = 0;
    let mut shortest_dm = 0;
    stack.push((from_x, from_y, 0, 0, 4, 0));

    while let Some((x, y, pos, dir, ret_dir, door_mask)) = stack.pop() {
        has_searched.set(x, y, pos);
        if shortest > 0 && pos > shortest {
            continue;
        }

        for d in dir..DIRS.len() {
            if d == ret_dir {
                continue;
            }

            let (dx, dy) = DIRS[d];
            let x2 = x + dx;
            let y2 = y + dy;
            let v = maze.get(x + dx, y + dy).unwrap();

            if *v == target {
                if shortest == 0 || shortest > pos + 1 {
                    shortest = pos + 1;
                    shortest_dm = door_mask;
                }

                continue;
            }

            let hs_len = *has_searched.get(x2, y2).unwrap();
            if hs_len > 0 && (pos+1) >= hs_len {
                continue;
            }

            match *v {
                b'A'..=b'Z' => {
                    let new_door_mask = door_mask | (1 << ((*v - b'A')+1) as u32);
                    stack.push((x, y, pos, d + 1, ret_dir, door_mask));
                    stack.push((x2, y2, pos + 1, 0, RET_DIRS[d], new_door_mask));
                }
                b'a'..=b'z' => {
                    stack.push((x, y, pos, d + 1, ret_dir, door_mask));
                    stack.push((x2, y2, pos + 1, 0, RET_DIRS[d], door_mask));
                }
                b'.' | b'@' => {
                    stack.push((x, y, pos, d + 1, ret_dir, door_mask));
                    stack.push((x2, y2, pos + 1, 0, RET_DIRS[d], door_mask));
                }
                _ => {}
            }
        }
    }

    (shortest, shortest_dm)
}

struct MazeTree {
    keys: Vec<MTKey>,
}

struct MTKey {
    x: usize,
    y: usize,
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
                    keys.push(MTKey{
                    x, y,
                    paths: Vec::with_capacity(32),
                })
            } else {
                break
            }
        }

        for i in 0..keys.len() {
            for j in (i+1)..keys.len() {
                let (dist, doors) = dfs(&maze, keys[i].x, keys[i].y, ALPHA[j]);
                if dist == 0 {
                    continue;
                }

                keys[i].paths.push(MTPath{
                    doors, to: j, len: dist,
                });
                keys[j].paths.push(MTPath{
                    doors, to: i, len: dist,
                });
            }
        }

        MazeTree {keys}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "#########
#b.A.@.a#
#########";

    const SAMPLE_2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const SAMPLE_3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const SAMPLE_4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const SAMPLE_5: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    #[test]
    fn test_dfs() {
        let maze = FixedGrid::<u8>::from_str(SAMPLE_1);

        let (sb_len, sb_doors) = dfs(&maze, 5, 1, b'b');
        let (sa_len, sa_doors) = dfs(&maze, 5, 1, b'a');

        assert_eq!(sb_doors, 2);
        assert_eq!(sb_len, 4);
        assert_eq!(sa_doors, 0);
        assert_eq!(sa_len, 2);
    }

    #[test]
    fn test_part1() {
        let maze1 = FixedGrid::<u8>::from_str(SAMPLE_1);
        let maze2 = FixedGrid::<u8>::from_str(SAMPLE_2);
        let maze3 = FixedGrid::<u8>::from_str(SAMPLE_3);
        let maze4 = FixedGrid::<u8>::from_str(SAMPLE_4);
        let maze5 = FixedGrid::<u8>::from_str(SAMPLE_5);

        assert_eq!(part1(&maze1), 8);
        assert_eq!(part1(&maze2), 86);
        assert_eq!(part1(&maze3), 132);
        assert_eq!(part1(&maze4), 136);
        assert_eq!(part1(&maze5), 81);
    }
}