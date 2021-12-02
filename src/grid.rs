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
        let cx = if x < 0 { x-self.chunk_width-1 } else {x} / self.chunk_width;
        let cy = if y < 0 { y-self.chunk_width-1 } else {y} / self.chunk_height;

        (cx, cy)
    }

    pub fn chunk(&self, ix: isize, iy: isize) -> Option<(&FixedGrid<T>, isize, isize)> {
        self.grids.iter()
            .find(|(_, ix2, iy2)| *ix2 == ix && *iy2 == iy)
            .map(|(g, ix2, iy2)| (g, ix2 * self.chunk_width, iy2 * self.chunk_height) )
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
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(y * self.width + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y * self.width + x)
    }
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(y * self.width + x)
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
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
    ) -> impl Iterator<Item = (usize, usize, &T)> {
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

    pub fn parse_str(s: &str) -> FixedGrid<char> {
        let width = s.lines().filter(|l| !l.is_empty()).next().unwrap().len();
        let height = s.lines().filter(|l| !l.is_empty()).count();
        let data = s.chars().filter(|c| *c != '\n' && *c != '\r').collect();

        FixedGrid::from(width, height, data)
    }
}