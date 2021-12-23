use smallvec::{SmallVec};
use std::cmp::{max, min};
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
    Point(1, -1, -1),
    Point(-1, 1, -1),
    Point(1, 1, -1),
    Point(-1, -1, 1),
    Point(1, -1, 1),
    Point(-1, 1, 1),
    Point(1, 1, 1),
];

const SUB_CENTERS2: [Point; 8] = [
    Point(-1, -1, -1),
    Point(0, -1, -1),
    Point(-1, 0, -1),
    Point(0, 0, -1),
    Point(-1, -1, 0),
    Point(0, -1, 0),
    Point(-1, 0, 0),
    Point(0, 0, 0),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point(pub isize, pub isize, pub isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cube(pub Point, pub Point);

impl<T> Octree<T> where T: Copy + std::cmp::PartialEq {
    pub fn reset(&mut self) {
        let old_factor = self.octants[0].factor;

        self.free_list.clear();
        self.octants.clear();
        self.octants.push(Octant{
            factor: old_factor,
            center: Point(0, 0, 0),
            subs: [0; 8],
            value: None,
        });
    }

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
                    self.free_sub_octants(index);
                    self.octants[index].subs = [0; 8];
                    break;
                }
                Err(sub_index) => {
                    // If the octant is a leaf and has the value V already, stop here.
                    if self.octants[index].value == v && self.octants[index].subs.iter().find(|v| **v > 0).is_none() {
                        break;
                    }

                    index = self.ensure_octant(index, sub_index);
                }
            }
        }
    }

    pub fn set_cube(&mut self, c: Cube, v: Option<T>) {
        let mut stack: SmallVec<[usize; 8]> = SmallVec::new();
        stack.push(0);
        while let Some(index) = stack.pop() {
            match self.octants[index].set_cube(c, v) {
                Ok(_) => {
                    self.free_sub_octants(index);
                    self.octants[index].subs = [0; 8];
                }
                Err(sub_indices) => {
                    // If the octant is a leaf and has the value V already, stop here.
                    if self.octants[index].value == v && self.octants[index].subs.iter().find(|v| **v > 0).is_none() {
                        continue;
                    }

                    for sub_index in sub_indices {
                        stack.push(self.ensure_octant(index, sub_index));
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
                    count += octant.value_weight();
                }
            }

            for sub in octant.subs {
                if sub > 0 {
                    stack.push(sub);
                }
            }
        }

        count
    }

    fn ensure_octant(&mut self, index: usize, sub_index: usize) -> usize {
        if self.octants[index].subs[sub_index] == 0 {
            // Collect data from current.
            let old_value = self.octants[index].value;
            let new_factor = self.octants[index].factor >> 1;
            let new_center = self.octants[index].sub_center(sub_index);

            // Move on to new octant
            if let Some(free_index) = self.free_list.pop() {
                self.octants[index].subs[sub_index] = free_index;

                // Populate new octant.
                let new_index = free_index;
                self.octants[new_index].subs = [0; 8];
                self.octants[new_index].value = old_value;
                self.octants[new_index].factor = new_factor;
                self.octants[new_index].center = new_center;

                new_index
            } else {
                self.octants[index].subs[sub_index] = self.octants.len();
                self.octants.push(Octant {
                    value: old_value,
                    subs: [0; 8],
                    factor: new_factor,
                    center: new_center,
                });

                self.octants.len() - 1
            }
        } else {
            self.octants[index].subs[sub_index]
        }
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

    fn set_cube(&mut self, c: Cube, value: Option<T>) -> Result<(), SmallVec<[usize; 8]>> {
        match self.check_coverage(&c) {
            Ok(_) => {
                self.value = value;
                Ok(())
            }
            Err(subs) => {
                Err(subs)
            }
        }
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

    /// Get the weight of the value, which is the volume of all non-sub values.
    fn value_weight(&self) -> usize {
        if self.factor == 0 {
            1
        } else {
            let sub_weight = (self.factor.pow(3)) as usize;
            sub_weight * self.subs.iter().filter(|v| **v == 0).count()
        }
    }

    fn cube(&self) -> Cube {
        let fac_point = Point(self.factor, self.factor, self.factor);
        let min = self.center - fac_point;
        let max = self.center + fac_point;
        Cube(min, max)
    }

    fn sub_center(&self, idx: usize) -> Point {
        let offset = if self.factor > 1 {
            SUB_CENTERS[idx] * Point(self.factor >> 1, self.factor >> 1, self.factor >> 1)
        } else {
            SUB_CENTERS2[idx]
        };

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
    fn check_coverage(&self, cube: &Cube) -> Result<(), SmallVec<[usize; 8]>> {
        if self.factor == 0 {
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

        self.x() >= min.x() && self.y() >= min.y() && self.z() >= min.z()
            && self.x() < max.x() && self.y() < max.y() && self.z() < max.z()
    }

    fn inside_cube_max(&self, cube: &Cube) -> bool {
        let Cube(min, max) = cube;

        self.x() >= min.x() && self.y() >= min.y() && self.z() >= min.z()
            && self.x() <= max.x() && self.y() <= max.y() && self.z() <= max.z()
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

    pub fn constrained(&self, other: &Cube) -> Option<Cube> {
        let Cube(s_min, s_max) = self;
        let Cube(o_min, o_max) = other;

        if self.overlaps(other) {
            Some(Cube(
                Point(
                    max(s_min.0, o_min.0),
                    max(s_min.1, o_min.1),
                    max(s_min.2, o_min.2),
                ),
                Point(
                    min(s_max.0, o_max.0),
                    min(s_max.1, o_max.1),
                    min(s_max.2, o_max.2),
                ),
            ))
        } else {
            None
        }
    }

    fn contained_by(&self, other: &Cube) -> bool {
        self.0.inside_cube(other) && self.1.inside_cube_max(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_are_correct() {
        let a = Octant::<()> { center: Point(4, 4, 4), factor: 4, subs: [0; 8], value: None };
        let b = Octant::<()> { center: Point(4, 4, 4), factor: 4, subs: [0; 8], value: None };

        assert_eq!(a.sub_center(7), Point(6, 6, 6));
    }

    #[test]
    fn can_set_single_value() {
        let mut octree = Octree::new(16);

        octree.set(Point(-3, -9, -12), Some(64));

        assert_eq!(octree, Octree::<i32> {
            octants: vec![Octant {
                value: None,
                factor: 16,
                center: Point(0, 0, 0),
                subs: [1, 0, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 8,
                center: Point(-8, -8, -8),
                subs: [0, 2, 0, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 4,
                center: Point(-4, -12, -12),
                subs: [0, 0, 0, 0, 0, 0, 0, 3],
            }, Octant {
                value: None,
                factor: 2,
                center: Point(-2, -10, -10),
                subs: [0, 0, 4, 0, 0, 0, 0, 0],
            }, Octant {
                value: None,
                factor: 1,
                center: Point(-3, -9, -11),
                subs: [0, 0, 0, 5, 0, 0, 0, 0],
            }, Octant {
                value: Some(64),
                factor: 0,
                center: Point(-3, -9, -12),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }],
            free_list: vec![],
        });
    }

    #[test]
    fn count_counts_singles() {
        let mut octree = Octree::new(16);

        octree.set(Point(-3, -9, -12), Some(64));
        octree.set(Point(-1, -1, -1), Some(66));
        octree.set(Point(-1, -1, -2), Some(57));
        octree.set(Point(-3, 9, -12), Some(66));
        octree.set(Point(-3, 9, -11), Some(65));
        octree.set(Point(-3, 9, -11), Some(65));
        octree.set(Point(-1, -1, -2), Some(62));
        octree.set(Point(0, 0, 0), Some(0));

        assert_eq!(octree.count(|v| *v >= 60 && *v < 70), 5);
    }

    #[test]
    fn can_poke_hole_and_count_correctly() {
        // Cube 0,0,0 to 16,16,16
        let mut octy = Octree::<i32> {
            octants: vec![Octant {
                value: None,
                factor: 16,
                center: Point(0, 0, 0),
                subs: [0, 0, 0, 0, 0, 0, 0, 1],
            }, Octant {
                value: Some(64),
                factor: 8,
                center: Point(8, 8, 8),
                subs: [0, 0, 0, 0, 0, 0, 0, 0],
            }],
            free_list: vec![],
        };

        assert_eq!(octy.count(|v| *v == 64), 4096);
        octy.set(Point(1, 2, 3), None);
        assert_eq!(octy.count(|v| *v == 64), 4095);
        octy.set(Point(1, 2, 3), None);
        assert_eq!(octy.count(|v| *v == 64), 4095);
        octy.set(Point(6, 7, 4), None);
        assert_eq!(octy.count(|v| *v == 64), 4094);
    }

    #[test]
    fn thin_bois_work() {
        let mut octy = Octree::new(16);
        octy.set_cube(Cube(
            Point(1, 2, 3),
            Point(1, 2, 33),
        ), Some(30));

        assert_eq!(octy.count(|_| true), 30);
    }

    #[test]
    fn can_set_cubes() {
        for x in -8..2 {
            for y in -4..3 {
                for z in -3..1 {
                    let mut ot = Octree::new(16);
                    ot.set_cube(
                        Cube(
                            Point(x, y, z),
                            Point(x + 8, y + 8, z + 8),
                        ),
                        Some(32),
                    );

                    ot.set_cube(
                        Cube(
                            Point(x + 4, y + 4, z + 4),
                            Point(x + 10, y + 10, z + 10),
                        ),
                        None,
                    );

                    assert_eq!(ot.count(|_| true), 448);
                }
            }
        }
    }
}