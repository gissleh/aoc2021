use std::collections::VecDeque;

#[derive(Clone)]
pub struct GridSet<T> {
    chunk_width: isize,
    chunk_height: isize,
    grids: Vec<(FixedGrid<T>, isize, isize)>,
}

impl<T> GridSet<T>
    where
        T: Clone + Copy,
{
    pub fn chunk_index(&self, x: isize, y: isize) -> (isize, isize) {
        let cx = if x < 0 { x - self.chunk_width - 1 } else { x } / self.chunk_width;
        let cy = if y < 0 { y - self.chunk_width - 1 } else { y } / self.chunk_height;

        (cx, cy)
    }

    pub fn chunk(&self, ix: isize, iy: isize) -> Option<(&FixedGrid<T>, isize, isize)> {
        self.grids.iter()
            .find(|(_, ix2, iy2)| *ix2 == ix && *iy2 == iy)
            .map(|(g, ix2, iy2)| (g, ix2 * self.chunk_width, iy2 * self.chunk_height))
    }
}

#[derive(Clone)]
pub struct FixedGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> FixedGrid<T>
{
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn data(&self) -> &[T] {
        return &self.data;
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y * self.width + x)
    }
    pub fn get_safe(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width {
            None
        } else if y >= self.height {
            None
        } else {
            self.data.get(y * self.width + x)
        }
    }
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(y * self.width + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y * self.width + x)
    }
    pub fn get_slice_mut(&mut self, x1: usize, x2: usize, y: usize) -> &mut [T] {
        let left = y * self.width + x1;
        let right = left + (x2 - x1);

        &mut self.data.as_mut_slice()[left..right]
    }
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(y * self.width + x)
    }
    pub fn iter(&self) -> impl Iterator<Item=(usize, usize, &T)> {
        let mut y = 0usize;
        let mut x = 0usize;

        self.data.iter().map(move |v| {
            let px = x;
            let py = y;

            x += 1;
            if x == self.width {
                x = 0;
                y += 1;
            }

            (px, py, v)
        })
    }
    pub fn limited_iter(
        &self,
        fx: usize,
        fy: usize,
        tx: usize,
        ty: usize,
    ) -> impl Iterator<Item=(usize, usize, &T)> {
        let w = tx - fx;
        let h = ty - fy;
        let first = (fy * self.width) + fx;

        assert!(fx < self.width);
        assert!(fy < self.height);
        assert!(tx <= self.width);
        assert!(ty <= self.height);

        (0..(w * h)).map(move |i| {
            let rx = i % w;
            let ry = i / w;

            unsafe {
                (
                    fx + rx,
                    fy + ry,
                    self.data.get_unchecked(first + (ry * self.width) + rx),
                )
            }
        })
    }
}

impl<T> FixedGrid<T>
{
    pub fn insert(&mut self, x: usize, y: usize, v: T) {
        self.data[y * self.width + x] = v;
    }
    pub fn empty() -> FixedGrid<T> {
        FixedGrid {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}

impl<T> FixedGrid<T>
    where
        T: Default + Clone,
{
    pub fn clear(&mut self) {
        self.data.fill(T::default());
    }

    pub fn blank(width: usize, height: usize) -> FixedGrid<T> {
        FixedGrid {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T> FixedGrid<T>
    where
        T: Clone + Copy,
{
    pub fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[y * self.width + x] = v;
    }

    pub unsafe fn set_unsafe(&mut self, x: usize, y: usize, v: T) {
        *self.data.get_unchecked_mut(y * self.width + x) = v;
    }

    pub fn set_slice(&mut self, x: usize, y: usize, src: &[T]) {
        let index = y * self.width + x;
        self.data[index..index + src.len()].copy_from_slice(src);
    }

    pub fn copy_from(&mut self, other: &FixedGrid<T>) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        self.data.copy_from_slice(&other.data);
    }

    pub fn new(width: usize, height: usize, def: T) -> FixedGrid<T> {
        FixedGrid {
            data: vec![def; width * height],
            width,
            height,
        }
    }

    pub fn from(width: usize, height: usize, data: Vec<T>) -> FixedGrid<T> {
        assert_eq!(data.len(), (width * height));

        FixedGrid {
            data,
            width,
            height,
        }
    }
}

impl<T> FixedGrid<T>
    where
        T: Eq,
{
    pub fn count(&self, v: T) -> usize {
        let mut count = 0;
        for v2 in self.data.iter() {
            if v == *v2 {
                count += 1;
            }
        }

        count
    }

    pub fn find(&self, v: T) -> Option<(usize, usize)> {
        self.iter()
            .find(|(_, _, v2)| v == **v2)
            .map(|(x, y, _)| (x, y))
    }
}

impl FixedGrid<u8>
{
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                unsafe {
                    print!("{}", *self.get_unchecked(x, y) as char);
                }
            }

            println!();
        }
    }

    pub fn from_str(s: &str) -> FixedGrid<u8> {
        let width = s.lines().filter(|l| !l.is_empty()).next().unwrap().len();
        let height = s.lines().filter(|l| !l.is_empty()).count();
        let data = s.bytes().filter(|c| *c != b'\n' && *c != b'\r').collect();

        FixedGrid::from(width, height, data)
    }
}

impl FixedGrid<char>
{
    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                unsafe {
                    print!("{}", self.get_unchecked(x, y));
                }
            }

            println!();
        }
    }

    pub fn from_str(s: &str) -> FixedGrid<char> {
        let width = s.lines().filter(|l| !l.is_empty()).next().unwrap().len();
        let height = s.lines().filter(|l| !l.is_empty()).count();
        let data = s.chars().filter(|c| *c != '\n' && *c != '\r').collect();

        FixedGrid::from(width, height, data)
    }
}

#[derive(Copy, Clone)]
pub struct TinyGrid<T, const W: usize, const S: usize> {
    data: [T; S],
}

impl<T, const W: usize, const S: usize> TinyGrid<T, W, S>  {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < W {
            self.data.get(y * W + x)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < W {
            self.data.get_mut(y * W + x)
        } else {
            None
        }
    }

    pub const fn new(data: [T; S]) -> TinyGrid<T, W, S> {
        TinyGrid{ data }
    }
}

impl<T, const W: usize, const S: usize> TinyGrid<T, W, S> where T: Copy {
    pub fn set(&mut self, x: usize, y: usize, v: T) {
        if x >= W {
            panic!("Set out of bounds")
        }

        *self.data.get_mut((y * W) + x).unwrap() = v;
    }
}

const OFFSETS_CARDINAL: [(usize, usize); 4] = [
    (0, !0),
    (!0, 0),
    (1, 0),
    (0, 1),
];

const OFFSETS_DIAGONAL: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
pub enum BFSStep<U> {
    Continue(U),
    Found(U),
    DeadEnd,
}

/// BFS struct is to keep some state between runs to avoid needless allocations.
#[derive(Clone)]
pub struct BFS<S> {
    visited: FixedGrid<bool>,
    queue: VecDeque<(usize, usize, usize, S)>,
    found_pos: Option<(usize, usize)>,
}

impl<S> BFS<S> where S: Clone + Default {
    /// Get the position where it was found in the last run.
    pub fn found_pos(&self) -> Option<(usize, usize)> {
        self.found_pos
    }

    /// Run BFS on this grid. This will reset all state, so is safe to call multiple times.
    pub fn run<'a, T>(&'a mut self, grid: &'a FixedGrid<T>, start_x: usize, start_y: usize, diagonal: bool, check: impl Fn(&'a T, (usize, usize), &S) -> BFSStep<S>) -> Option<(&'a T, usize, S)> {
        if self.visited.width < grid.width || self.visited.height < grid.height {
            self.visited = FixedGrid::blank(grid.width, grid.height);
        } else {
            self.visited.clear();
        }

        self.found_pos = None;

        self.queue.clear();
        self.queue.push_back((start_x, start_y, 0, S::default()));

        let offsets: &[(usize, usize)] = if diagonal { &OFFSETS_DIAGONAL } else { &OFFSETS_CARDINAL };

        while let Some((x, y, l, state)) = self.queue.pop_front() {
            self.visited.set(x, y, true);
            let v = grid.get(x, y).unwrap();

            match check(v, (x, y), &state) {
                BFSStep::Continue(new_state) => {
                    for (x_offset, y_offset) in offsets.iter() {
                        let x2 = x.wrapping_add(*x_offset);
                        if x2 >= grid.width() {
                            continue;
                        }
                        let y2 = y.wrapping_add(*y_offset);
                        if y2 >= grid.height() {
                            continue;
                        }
                        if *self.visited.get(x2, y2).unwrap() {
                            continue;
                        }

                        self.queue.push_back((x2, y2, l + 1, new_state.clone()));
                    }
                }
                BFSStep::Found(new_state) => {
                    self.found_pos = Some((x, y));
                    return Some((v, l, new_state));
                }
                BFSStep::DeadEnd => {
                    // Do nothing.
                }
            }
        }

        None
    }

    pub fn new() -> BFS<S> {
        BFS {
            visited: FixedGrid::empty(),
            queue: VecDeque::with_capacity(64),
            found_pos: None,
        }
    }
}

pub fn valid_offsets(diagonal: bool, x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item=(usize, usize)> {
    let offsets = if diagonal { OFFSETS_DIAGONAL.as_slice() } else { OFFSETS_CARDINAL.as_slice() };

    offsets.iter().map(move |(xo, yo)| (
        x.wrapping_add(*xo),
        y.wrapping_add(*yo),
    )).filter(move |(x, y)| *x < w && *y < h)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const TEST_GRID: &str = "
######################
#y...............#xxx#
#.#x##########.#.#x#x#
#.#....x.......#.#.#x#
#x###.#.###.##.#.#.#x#
#.##..#.###.##.#...#x#
#.#..##.###.########.#
#.#.###..............#
#x#.#................#
#.#.#.###............#
#.#.#.#...#.##.#######
#.....#.###.##....####
##...##.#...#..##....#
######################
";

    #[test]
    pub fn test_bfs_nostate() {
        let mut bfs = BFS::<()>::new();
        let grid = FixedGrid::<u8>::from_str(&TEST_GRID);

        let res = bfs.run(&grid, 12, 8, false, checker_without_state);

        assert!(res.is_some());
        let (x, len, _) = res.unwrap();
        assert_eq!(*x, b'y');
        assert_eq!(len, 18);
    }

    #[test]
    pub fn test_bfs_state() {
        let mut bfs = BFS::<u32>::new();
        let grid = FixedGrid::<u8>::from_str(&TEST_GRID);

        let res = bfs.run(&grid, 12, 8, false, checker_with_state);

        assert!(res.is_some());
        let (x, len, s) = res.unwrap();
        assert_eq!(*x, b'y');
        assert_eq!(len, 18);
        assert_eq!(s, 2);
    }

    fn checker_without_state(v: &u8, _: (usize, usize), _: &()) -> BFSStep<()> {
        match *v {
            b'#' => BFSStep::DeadEnd,
            b'y' => BFSStep::Found(()),
            _ => BFSStep::Continue(()),
        }
    }

    fn checker_with_state(v: &u8, _: (usize, usize), s: &u32) -> BFSStep<u32> {
        match *v {
            b'#' => BFSStep::DeadEnd,
            b'x' => BFSStep::Continue(*s + 1),
            b'y' => BFSStep::Found(*s),
            _ => BFSStep::Continue(*s),
        }
    }
}