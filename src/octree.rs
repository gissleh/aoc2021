use smallvec::{SmallVec, smallvec};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul};

const SUB_EDGES: [Cube; 8] = [
    Cube(Point(-1, -1, -1), Point(0, 0, 0)),
    Cube(Point(0, -1, -1), Point(1, 0, 0)),
    Cube(Point(-1, 0, -1), Point(0, 1, 0)),
    Cube(Point(0, 0, -1), Point(1, 1, 0)),
    Cube(Point(-1, -1, 0), Point(0, 0, 1)),
    Cube(Point(0, -1, 0), Point(1, 0, 1)),
    Cube(Point(-1, 0, 0), Point(0, 1, 1)),
    Cube(Point(0, 0, 0), Point(1, 1, 1)),
];

const SUB_CENTERS: [Point; 8] = [
    Point(-1, -1, -1),
    Point(0, -1, -1),
    Point(-1, 0, -1),
    Point(0, 0, -1),
    Point(0, 0, 1),
    Point(1, 0, 1),
    Point(0, 1, 1),
    Point(1, 1, 1),
];

// -z, -y, -x = 000
// -z, -y, x = 001
// -z, y, -x = 010
// -z, y, x = 011
// z, -y, -x = 100
// z, -y, x = 101
// z, y, -x = 110
// z, y, x = 111

#[derive(Debug, Eq, PartialEq)]
pub struct Octree<T> {
    octants: Vec<Octant<T>>,
    free_list: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct Octant<T> {
    value: Option<T>,
    factor: isize,
    center: Point,
    subs: [usize; 8],
}

pub struct Batch<T> {
    min: Point,
    max: Point,
    value: Option<T>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point(isize, isize, isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cube(Point, Point);

impl<T> Octree<T> where T: Copy + std::cmp::PartialEq {
    pub fn get(&self, p: Point) -> Option<T> {
        let mut index = 0;
        loop {
            match self.octants[index].get(p) {
                Ok(v) => {
                    return v;
                }
                Err(new_index) => {
                    index = new_index;
                }
            }
        }
    }

    pub fn set(&mut self, p: Point, v: Option<T>) {
        let mut index = 0;
        loop {
            match self.octants[index].set(p, v) {
                Ok(_) => {
                    break;
                }
                Err(sub_index) => {
                    if self.octants[index].subs[sub_index] == 0 {
                        // Collect data from current.
                        let old_value = self.octants[index].value;
                        let new_factor = self.octants[index].factor >> 1;
                        let new_center = self.octants[index].sub_center(sub_index);

                        // No need to create a more specific if the value here is exact.
                        if old_value == v {
                            return;
                        }

                        // Move on to new octant
                        if let Some(free_index) = self.free_list.pop() {
                            self.octants[index].subs[sub_index] = free_index;
                            index = free_index;

                            // Populate new octant.
                            self.octants[index].subs = [0; 8];
                            self.octants[index].value = old_value;
                            self.octants[index].center = new_center;
                            self.octants[index].factor = new_factor;
                        } else {
                            self.octants[index].subs[sub_index] = self.octants.len();
                            index = self.octants.len();
                            self.octants.push(Octant {
                                value: old_value,
                                subs: [0; 8],
                                factor: new_factor,
                                center: new_center,
                            });
                        }
                    } else {
                        index = self.octants[index].subs[sub_index];
                    }
                }
            }
        }
    }

    pub fn count(&self, callback: impl Fn(&T) -> bool) -> usize {
        let mut stack: SmallVec<[usize; 64]> = SmallVec::new();
        let mut count = 0;
        stack.push(0);

        while let Some(i) = stack.pop() {
            let octant = &self.octants[i];

            if let Some(v) = &octant.value {
                if callback(v) {
                    count += octant.cube().volume() as usize
                }
            } else {
                stack.extend(octant.subs.iter().copied().filter(|v| *v > 0));
            }
        }

        count
    }

    /// Free an octant and all its children. This assumes no octant is pointed at it.
    fn free_octant(&mut self, index: usize) {
        self.free_list.push(index);
        self.free_sub_octants(index);
    }

    /// Free an octant's children recursively. This assumes no octant is pointed at it.
    fn free_sub_octants(&mut self, index: usize) {
        for index in self.octants[index].subs {
            if index > 0 {
                self.free_octant(index)
            }
        }
    }

    pub fn new(factor: isize) -> Octree<T> {
        assert_eq!(factor.count_ones(), 1);

        Octree {
            octants: vec![
                Octant::new(factor, Point(0, 0, 0), None),
            ],
            free_list: Vec::with_capacity(16),
        }
    }
}

impl<T> Octant<T> where T: Copy + std::cmp::PartialEq {
    fn sub_index(&self, p: Point) -> usize {
        (if p.x() < self.center.x() { 0 } else { 1 }
            | if p.y() < self.center.y() { 0 } else { 2 }
            | if p.z() < self.center.z() { 0 } else { 4 })
    }

    fn set(&mut self, p: Point, value: Option<T>) -> Result<(), usize> {
        if self.factor > 0 {
            Err(self.sub_index(p))
        } else {
            self.value = value;
            Ok(())
        }
    }

    fn get(&self, p: Point) -> Result<Option<T>, usize> {
        if self.factor > 0 {
            let index = self.sub_index(p);

            if self.subs[index] == 0 {
                Ok(self.value)
            } else {
                Err(self.subs[index])
            }
        } else {
            Ok(self.value)
        }
    }

    fn cube(&self) -> Cube {
        let fac_point = Point(self.factor, self.factor, self.factor);
        let min = self.center - fac_point;
        let max = self.center + fac_point;
        Cube(min, max)
    }

    fn sub_center(&self, idx: usize) -> Point {
        let offset = Point(self.factor >> 1, self.factor >> 1, self.factor >> 1) * SUB_CENTERS[idx];
        self.center + offset
    }

    fn sub_cubes<'a>(&'a self) -> impl Iterator<Item=Cube> + 'a {
        let fac_point = Point(self.factor, self.factor, self.factor);

        SUB_EDGES.iter().map(move |Cube(min, max)| Cube(
            self.center + (*min * fac_point),
            self.center + (*max * fac_point),
        ))
    }

    /// covered_by returns true if the entire octant is covered by this cube. Otherwise, it returns
    /// a list of up to 8 local indices that are **fully or partially** covered.
    fn covered_by(&self, cube: &Cube) -> Result<(), SmallVec<[usize; 8]>> {
        if self.factor == 1 {
            // The atom octant holds only the value at its center.
            if self.center.inside_cube(&cube) {
                Ok(())
            } else {
                Err(SmallVec::new())
            }
        } else if self.cube().contained_by(cube) {
            Ok(())
        } else {
            let mut vec: SmallVec<[usize; 8]> = SmallVec::new();
            for (i, sub_cube) in self.sub_cubes().enumerate() {
                if sub_cube.overlaps(cube) {
                    vec.push(i);
                }
            }

            Err(vec)
        }
    }

    fn new(factor: isize, center: Point, value: Option<T>) -> Octant<T> {
        Octant {
            subs: [0; 8],
            factor,
            center,
            value,
        }
    }
}

impl Point {
    pub fn x(&self) -> isize {
        self.0
    }
    pub fn y(&self) -> isize {
        self.1
    }
    pub fn z(&self) -> isize {
        self.2
    }

    fn inside_cube(&self, cube: &Cube) -> bool {
        let Cube(min, max) = cube;

        self.x() >= min.x() && self.y() >= min.y() && self.x() >= min.z()
            && self.x() < max.x() && self.y() < max.y() && self.x() < max.z()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Cube {
    fn overlaps(&self, other: &Cube) -> bool {
        (self.1.x() > other.0.x())
            && (self.0.x() < other.1.x())
            && (self.1.y() > other.0.y())
            && (self.0.y() < other.1.y())
            && (self.1.z() > other.0.z())
            && (self.0.z() < other.1.z())
    }

    fn volume(&self) -> isize {
        (self.1.0 - self.0.0)
            * (self.1.1 - self.0.1)
            * (self.1.2 - self.0.2)
    }

    fn contained_by(&self, other: &Cube) -> bool {
        self.0.inside_cube(other) && self.1.inside_cube(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_works() {
        let mut octree = Octree::new(16);

        octree.set(Point(-3, -9, -12), Some(64));
        octree.set(Point(-1, -1, -1), Some(572));
        octree.set(Point(-1, -1, -2), Some(772));
        octree.set(Point(0, 0, 0), Some(0));
        octree.set(Point(4, 3, 1), None);
        octree.set(Point(0, 0, 0), None);

        println!("{:#?}", octree);

        assert_eq!(octree, Octree::<i32> {
            octants: vec![Octant {
                value: None,
                factor: 16,
                center: Point(0, 0, 0),
                subs: [1, 0, 0, 0, 0, 0, 0, 11],
            }, Octant {
                value: None,
                factor: 8,
                center: Point(-8, -8, -8),
                subs: [0, 2, 0, 0, 0, 0, 0, 6],
            }, Octant {
                value: None,
                factor: 4,
                center: Point(-8, -12, -12),
                subs: [0, 0, 0, 0, 0, 0, 0, 3],
            }, Octant {
                value: None,
                factor: 2,
                center: Point(-6, -10, -10),
                subs: [0, 0, 0, 4, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 1,
                center:
                Point(-6, -10, -11),
                subs: [0, 0, 0, 5, 0, 0, 0, 0],
            }, Octant {
                value: Some(64),
                factor: 0,
                center:
                Point(-6, -10, -11),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 4,
                center:
                Point(-4, -4, -4),
                subs: [0, 0, 0, 0, 0, 0, 0, 7],
            }, Octant {
                value: None,
                factor: 2,
                center:
                Point(-2, -2, -2),
                subs: [0, 0, 0, 0, 0, 0, 0, 8],
            }, Octant {
                value: None,
                factor: 1,
                center:
                Point(-1, -1, -1),
                subs: [0, 0, 0, 10, 0, 0, 0, 9],
            }, Octant {
                value: Some(572),
                factor: 0,
                center: Point(-1, -1, -1),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: Some(772),
                factor: 0,
                center: Point(-1, -1, -1),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 0,
                center: Point(8, 8, 8),
                subs: [12, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 0,
                center: Point(4, 4, 4),
                subs: [13, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 0,
                center: Point(2, 2, 2),
                subs: [14, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 0,
                center: Point(1, 1, 1),
                subs: [15, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 0,
                center: Point(0, 0, 0),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }],
            free_list: vec![],
        });

        assert_eq!(octree.get(Point(-3, -9, -12)), Some(64));
        assert_eq!(octree.get(Point(-1, -1, -1)), Some(572));
        assert_eq!(octree.get(Point(-1, -1, -2)), Some(772));
        assert_eq!(octree.get(Point(-1, -1, -3)), None);
        assert_eq!(octree.get(Point(-1123, -166, -33)), None);
        assert_eq!(octree.get(Point(0, 0, 0)), None);
        assert_eq!(octree.get(Point(1,3,2)), None);
    }
}