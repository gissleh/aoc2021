use common::aoc::{print_result, run_many, print_time_cold};
use common::grid::{FixedGrid, BFS, BFSStep};
use smallvec::{SmallVec, smallvec};

fn main() {
    let input = include_str!("../input/year2019-day20.txt");

    let (input, dur_p, dur_pc) = run_many(1000, || Maze::parse(input));
    let (res_p1, dur_p1, dur_p1c) = run_many(1, || part1(&input));
    assert_eq!(res_p1, 400);
    let (res_p2, dur_p2, dur_p2c) = run_many(1, || part2(&input));

    print_result("P1", res_p1);
    print_result("P2", res_p2);

    print_time_cold("Parse", dur_p, dur_pc);
    print_time_cold("P1", dur_p1, dur_p1c);
    print_time_cold("P2", dur_p2, dur_p2c);
    print_time_cold("Total", dur_p + dur_p1 + dur_p2, dur_pc + dur_p1c + dur_p2c);
}

fn part1(input: &Maze) -> usize {
    let mut bfs = BFS::new();
    let (sx, sy) = input.start_position;
    let res = bfs.run(&input.grid, sx, sy, false, |tile, _, _| {
        match tile {
            Tile::End => BFSStep::Found(()),
            Tile::Wall => BFSStep::DeadEnd,
            Tile::Floor => BFSStep::Continue(()),
            Tile::WarpOuter(x, y, ..) => BFSStep::Warp(*x, *y, ()),
            Tile::WarpInner(x, y, ..) => BFSStep::Warp(*x, *y, ()),
        }
    });

    if let Some((_, l, _)) = res {
        l
    } else {
        0
    }
}

fn part2(input: &Maze) -> usize {
    let mut bfs = BFS::new();
    let (sx, sy) = input.start_position;
    let res = bfs.run_multilevel(&input.grid, sx, sy, 1000, false, |tile, pos, level, last_warp| {
        if pos == *last_warp {
            return BFSStep::DeadEnd;
        }

        match tile {
            Tile::End => if level == 1000 {BFSStep::Found(*last_warp)} else {BFSStep::DeadEnd},
            Tile::Wall => BFSStep::DeadEnd,
            Tile::Floor => BFSStep::Continue(*last_warp),
            Tile::WarpOuter(x, y, wx, wy) => BFSStep::WarpLevel(*x, *y, -1, (*wx, *wy)),
            Tile::WarpInner(x, y, wx, wy) => BFSStep::WarpLevel(*x, *y, 1, (*wx, *wy)),
        }
    });

    if let Some((_, l, _)) = res {
        l
    } else {
        0
    }
}

struct Maze {
    grid: FixedGrid<Tile>,
    start_position: (usize, usize),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    WarpInner(usize, usize, usize, usize),
    WarpOuter(usize, usize, usize, usize),
    End,
}

impl Maze {
    #[allow(dead_code)]
    fn print(&self) {
        let mut warps = Vec::with_capacity(32);

        println!("Start: {:?}", self.start_position);
        for line in self.grid.lines() {
            for tile in line.iter() {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::Floor => print!("."),
                    Tile::WarpOuter(x, y, ..) => {
                        warps.push((x, y));
                        print!("^");
                    },
                    Tile::WarpInner(x, y, ..) => {
                        warps.push((x, y));
                        print!("V");
                    },
                    Tile::End => print!("E"),
                }
            }

            for (x, y) in warps.iter() {
                print!(" {},{}", *x, *y);
            }
            warps.clear();

            println!();
        }
    }

    fn parse(input: &str) -> Maze {
        let raw_grid = FixedGrid::<u8>::from_str(input);
        let mut warp_positions: Vec<([u8; 2], SmallVec<[(usize, usize); 4]>)> = Vec::with_capacity(64);
        let mut start_position = (0usize, 0usize);
        let mut grid = FixedGrid::new(raw_grid.width(), raw_grid.height(), Tile::Wall);

        let middle = (grid.width() / 2, grid.height() / 2);

        for (x, y, v) in raw_grid.iter() {
            match *v {
                b'A'..=b'Z' => {
                    let mut key = [*v, 0u8];
                    let mut warp_pos = (0, 0);
                    let mut target_pos = (0, 0);

                    if let Some(v2) = raw_grid.get(x + 1, y) {
                        if *v2 >= b'A' && *v2 <= b'Z' {
                            key[1] = *v2;

                            if raw_grid.has(x - 1, y, b'.') {
                                target_pos = (x - 1, y);
                                warp_pos = (x, y);
                            } else {
                                target_pos = (x + 2, y);
                                warp_pos = (x + 1, y);
                            }
                        }
                    }
                    if let Some(v2) = raw_grid.get(x, y + 1) {
                        if *v2 >= b'A' && *v2 <= b'Z' {
                            key[1] = *v2;

                            if raw_grid.has(x, y - 1, b'.') {
                                target_pos = (x, y - 1);
                                warp_pos = (x, y);
                            } else {
                                target_pos = (x, y + 2);
                                warp_pos = (x, y + 1);
                            }
                        }
                    }
                    if key[1] == 0 {
                        continue;
                    }

                    match &key {
                        b"AA" => {
                            start_position = target_pos
                        }
                        b"ZZ" => {
                            grid[target_pos] = Tile::End;
                            println!("{:?}", target_pos);
                        }
                        _ => {
                            if let Some((_, vec)) = warp_positions.iter_mut().find(|(k, _)| k == &key) {
                                vec.push(target_pos);
                                vec.push(warp_pos);
                            } else {
                                warp_positions.push((key, smallvec![target_pos, warp_pos]));
                            }
                        }
                    }
                }
                b'.' => {
                    grid[(x, y)] = Tile::Floor;
                }
                _ => {}
            }
        }

        for (_, list) in warp_positions.into_iter() {
            assert_eq!(list.len(), 4);
            let (ax, ay) = list[0];
            let (bx, by) = list[1];
            let (cx, cy) = list[2];
            let (dx, dy) = list[3];
            if manhattan_distance((ax, ay), middle) < manhattan_distance((cx, cy), middle) {
                grid.set(bx, by, Tile::WarpInner(cx, cy, dx, dy));
                grid.set(dx, dy, Tile::WarpOuter(ax, ay, bx, by));
            } else {
                grid.set(bx, by, Tile::WarpOuter(cx, cy, dx, dy));
                grid.set(dx, dy, Tile::WarpInner(ax, ay, bx, by));
            }
        }

        Maze {
            grid,
            start_position,
        }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    abs_diff(a.0, b.0) + abs_diff(a.1, b.1)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const SAMPLE_1: &str = "         A-----------
         A-----------
  #######.#########--
  #######.........#--
  #######.#######.#--
  #######.#######.#--
  #######.#######.#--
  #####  B    ###.#--
BC...##  C    ###.#--
  ##.##       ###.#--
  ##...DE  F  ###.#--
  #####    G  ###.#--
  #########.#####.#--
DE..#######...###.#--
  #.#########.###.#--
FG..#########.....#--
  ###########.#####--
             Z-------
             Z-------
";

    const SAMPLE_2: &str = "                    A---------------
                    A---------------
   #################.#############--
   #.#...#...................#.#.#--
   #.#.#.###.###.###.#########.#.#--
   #.#.#.......#...#.....#.#.#...#--
   #.#########.###.#####.#.#.###.#--
   #.............#.#.....#.......#--
   ###.###########.###.#####.#.#.#--
   #.....#        A   C    #.#.#.#--
   #######        S   P    #####.#--
   #.#...#                 #......VT
   #.#.#.#                 #.#####--
   #...#.#               YN....#.#--
   #.###.#                 #####.#--
 DI....#.#                 #.....#--
   #####.#                 #.###.#--
 ZZ......#               QG....#..AS
   ###.###                 #######--
 JO..#.#.#                 #.....#--
   #.#.#.#                 ###.#.#--
   #...#..DI             BU....#..LF
   #####.#                 #.#####--
 YN......#               VT..#....QG
   #.###.#                 #.###.#--
   #.#...#                 #.....#--
   ###.###    J L     J    #.#.###--
   #.....#    O F     P    #.#...#--
   #.###.#####.#.#####.#####.###.#--
   #...#.#.#...#.....#.....#.#...#--
   #.#####.###.###.#.#.#########.#--
   #...#.#.....#...#.#.#.#.....#.#--
   #.###.#####.###.###.#.#.#######--
   #.#.........#...#.............#--
   #########.###.###.#############--
            B   J   C---------------
            U   P   P---------------
";

    #[test]
    pub fn test_part1() {
        //let mut maze_1 = Maze::parse(SAMPLE_1);
        let mut maze_2 = Maze::parse(SAMPLE_2);

        //maze_1.print();
        maze_2.print();

        //assert_eq!(part1(&maze_1), 23);
        assert_eq!(part1(&maze_2), 58);
    }
}