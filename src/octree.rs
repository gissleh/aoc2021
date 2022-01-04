use smallvec::{SmallVec};
use std::cmp::{max, min};
use std::ops::{Add, Sub, Mul};

const SUB_EDGES: [IndexCube; 8] = [
    IndexCube(IndexPoint(-1, -1, -1), IndexPoint(0, 0, 0)),
    IndexCube(IndexPoint(0, -1, -1), IndexPoint(1, 0, 0)),
    IndexCube(IndexPoint(-1, 0, -1), IndexPoint(0, 1, 0)),
    IndexCube(IndexPoint(0, 0, -1), IndexPoint(1, 1, 0)),
    IndexCube(IndexPoint(-1, -1, 0), IndexPoint(0, 0, 1)),
    IndexCube(IndexPoint(0, -1, 0), IndexPoint(1, 0, 1)),
    IndexCube(IndexPoint(-1, 0, 0), IndexPoint(0, 1, 1)),
    IndexCube(IndexPoint(0, 0, 0), IndexPoint(1, 1, 1)),
];

const SUB_CENTERS: [IndexPoint; 8] = [
    IndexPoint(-1, -1, -1),
    IndexPoint(1, -1, -1),
    IndexPoint(-1, 1, -1),
    IndexPoint(1, 1, -1),
    IndexPoint(-1, -1, 1),
    IndexPoint(1, -1, 1),
    IndexPoint(-1, 1, 1),
    IndexPoint(1, 1, 1),
];

const SUB_CENTERS2: [IndexPoint; 8] = [
    IndexPoint(-1, -1, -1),
    IndexPoint(0, -1, -1),
    IndexPoint(-1, 0, -1),
    IndexPoint(0, 0, -1),
    IndexPoint(-1, -1, 0),
    IndexPoint(0, -1, 0),
    IndexPoint(-1, 0, 0),
    IndexPoint(0, 0, 0),
];


#[derive(Debug, Eq, PartialEq)]
pub struct Octree<T> {
    root: Octant<T>,
    factor: isize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Octant<T> {
    Leaf(Option<T>),
    Branch(Box<[Octant<T>; 8]>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IndexPoint(pub isize, pub isize, pub isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IndexCube(pub IndexPoint, pub IndexPoint);

impl<T> Octree<T> where T: Copy + std::cmp::PartialEq {
    pub fn get(&self, p: IndexPoint) -> Option<T> {
        self.root.get(IndexPoint(0, 0, 0), p, self.factor)
    }

    pub fn set(&mut self, p: IndexPoint, value: Option<T>) {
        self.root.set(IndexPoint(0, 0, 0), p, self.factor, value)
    }

    pub fn set_cube(&mut self, cube: IndexCube, value: Option<T>) {
        self.root.set_cube(cube, IndexPoint(0, 0, 0), self.factor, value)
    }

    pub fn count(&self, callback: impl Fn(&T) -> bool) -> usize {
        let mut stack: SmallVec<[(&Octant<T>, isize); 64]> = SmallVec::new();
        let mut count = 0;
        stack.push((&self.root, 8 * self.factor * self.factor * self.factor));

        while let Some((octant, factor)) = stack.pop() {
            match octant {
                Octant::Leaf(v) => {
                    if let Some(v) = v {
                        if callback(v) {
                            count += factor as usize;
                        }
                    }
                }
                Octant::Branch(subs) => {
                    let sub_factor = factor >> 3;
                    stack.extend(subs.iter().map(|o| (o, sub_factor)));
                }
            }
        }

        count
    }

    pub fn new(factor: isize, root_value: Option<T>) -> Octree<T> {
        assert_eq!(factor.count_ones(), 1);

        Octree {
            factor,
            root: Octant::Leaf(root_value),
        }
    }
}

impl<T> Octant<T> where T: Copy + std::cmp::PartialEq {
    fn subdivide(&mut self) {
        match self {
            Octant::Leaf(value) => {
                *self = Octant::Branch(
                    Box::new([
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                        Octant::Leaf(*value),
                    ])
                )
            }
            Octant::Branch(_) => {}
        }
    }

    fn set(&mut self, center: IndexPoint, p: IndexPoint, factor: isize, value: Option<T>) {
        if factor == 0 {
            *self = Octant::Leaf(value);
        } else {
            self.subdivide();
            if let Octant::Branch(subs) = self {
                let si = p.calc_sub_index(center);
                let center = center.sub_center(si, factor);

                subs[si].set(center, p, factor >> 1, value);
            }
        }
    }

    fn set_cube(&mut self, cube: IndexCube, c: IndexPoint, factor: isize, value: Option<T>) {
        match self.check_coverage(&cube, c, factor) {
            Ok(_) => {
                *self = Octant::Leaf(value)
            }
            Err(sub_indices) => {
                self.subdivide();
                if let Octant::Branch(subs) = self {
                    for i in sub_indices {
                        let c = c.sub_center(i, factor);

                        subs[i].set_cube(cube, c, factor >> 1, value)
                    }
                }
            }
        }
    }

    fn get(&self, c: IndexPoint, p: IndexPoint, factor: isize) -> Option<T> {
        match self {
            Octant::Leaf(v) => {
                *v
            }
            Octant::Branch(subs) => {
                let i = p.calc_sub_index(c);
                let c = c.sub_center(i, factor);

                subs[i].get(c, p, factor >> 1)
            }
        }
    }

    /// covered_by returns true if the entire octant is covered by this cube. Otherwise, it returns
    /// a list of up to 8 local indices that are **fully or partially** covered.
    fn check_coverage(&self, cube: &IndexCube, center: IndexPoint, factor: isize) -> Result<(), SmallVec<[usize; 8]>> {
        if factor == 0 {
            // The atom octant holds only the value at its center.
            if center.inside_cube(&cube) {
                Ok(())
            } else {
                Err(SmallVec::new())
            }
        } else if IndexCube::for_octant(center, factor).contained_by(cube) {
            Ok(())
        } else {
            let mut vec: SmallVec<[usize; 8]> = SmallVec::new();
            for (i, sub_cube) in IndexCube::for_sub_octants(center, factor).enumerate() {
                if sub_cube.overlaps(cube) {
                    vec.push(i);
                }
            }

            Err(vec)
        }
    }
}

impl IndexPoint {
    pub fn x(&self) -> isize {
        self.0
    }
    pub fn y(&self) -> isize {
        self.1
    }
    pub fn z(&self) -> isize {
        self.2
    }

    fn calc_sub_index(&self, center: IndexPoint) -> usize {
        (if self.x() < center.x() { 0 } else { 1 }
            | if self.y() < center.y() { 0 } else { 2 }
            | if self.z() < center.z() { 0 } else { 4 })
    }

    fn sub_center(&self, index: usize, factor: isize) -> Self {
        let offset = if factor > 1 {
            SUB_CENTERS[index] * IndexPoint(factor >> 1, factor >> 1, factor >> 1)
        } else {
            SUB_CENTERS2[index]
        };

        *self + offset
    }

    fn inside_cube(&self, cube: &IndexCube) -> bool {
        let IndexCube(min, max) = cube;

        self.x() >= min.x() && self.y() >= min.y() && self.z() >= min.z()
            && self.x() < max.x() && self.y() < max.y() && self.z() < max.z()
    }

    fn inside_cube_max(&self, cube: &IndexCube) -> bool {
        let IndexCube(min, max) = cube;

        self.x() >= min.x() && self.y() >= min.y() && self.z() >= min.z()
            && self.x() <= max.x() && self.y() <= max.y() && self.z() <= max.z()
    }
}

impl Add for IndexPoint {
    type Output = IndexPoint;

    fn add(self, rhs: Self) -> Self::Output {
        IndexPoint(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for IndexPoint {
    type Output = IndexPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        IndexPoint(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for IndexPoint {
    type Output = IndexPoint;

    fn mul(self, rhs: Self) -> Self::Output {
        IndexPoint(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl IndexCube {
    fn overlaps(&self, other: &IndexCube) -> bool {
        (self.1.x() > other.0.x())
            && (self.0.x() < other.1.x())
            && (self.1.y() > other.0.y())
            && (self.0.y() < other.1.y())
            && (self.1.z() > other.0.z())
            && (self.0.z() < other.1.z())
    }

    pub fn constrained(&self, other: &IndexCube) -> Option<IndexCube> {
        let IndexCube(s_min, s_max) = self;
        let IndexCube(o_min, o_max) = other;

        if self.overlaps(other) {
            Some(IndexCube(
                IndexPoint(
                    max(s_min.0, o_min.0),
                    max(s_min.1, o_min.1),
                    max(s_min.2, o_min.2),
                ),
                IndexPoint(
                    min(s_max.0, o_max.0),
                    min(s_max.1, o_max.1),
                    min(s_max.2, o_max.2),
                ),
            ))
        } else {
            None
        }
    }

    fn for_octant(center: IndexPoint, factor: isize) -> IndexCube {
        let fac_point = IndexPoint(factor, factor, factor);

        IndexCube(
            center - fac_point,
            center + fac_point,
        )
    }

    fn for_sub_octants(center: IndexPoint, factor: isize) -> impl Iterator<Item=IndexCube> {
        let fac_point = IndexPoint(factor, factor, factor);

        SUB_EDGES.iter().map(move |IndexCube(min, max)| IndexCube(
            center + (*min * fac_point),
            center + (*max * fac_point),
        ))
    }

    fn contained_by(&self, other: &IndexCube) -> bool {
        self.0.inside_cube(other) && self.1.inside_cube_max(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_single_value() {
        let mut octree = Octree::new(16, None);

        octree.set(IndexPoint(-3, -9, -12), Some(64));

        assert_eq!(octree, Octree::<i32> {
            factor: 16,
            root: Octant::Branch(Box::new([
                Octant::Branch(Box::new([
                    Octant::Leaf(None),
                    Octant::Branch(Box::new([
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Leaf(None),
                        Octant::Branch(Box::new([
                            Octant::Leaf(None),
                            Octant::Leaf(None),
                            Octant::Branch(Box::new([
                                Octant::Leaf(None),
                                Octant::Leaf(None),
                                Octant::Leaf(None),
                                Octant::Leaf(Some(64)),
                                Octant::Leaf(None),
                                Octant::Leaf(None),
                                Octant::Leaf(None),
                                Octant::Leaf(None),
                            ])),
                            Octant::Leaf(None),
                            Octant::Leaf(None),
                            Octant::Leaf(None),
                            Octant::Leaf(None),
                            Octant::Leaf(None),
                        ])),
                    ])),
                    Octant::Leaf(None),
                    Octant::Leaf(None),
                    Octant::Leaf(None),
                    Octant::Leaf(None),
                    Octant::Leaf(None),
                    Octant::Leaf(None),
                ])),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
            ])),
        });
    }

    #[test]
    fn count_counts_singles() {
        let mut octree = Octree::new(16, None);

        octree.set(IndexPoint(-3, -9, -12), Some(64));
        octree.set(IndexPoint(-1, -1, -1), Some(66));
        octree.set(IndexPoint(-1, -1, -2), Some(57));
        octree.set(IndexPoint(-3, 9, -12), Some(66));
        octree.set(IndexPoint(-3, 9, -11), Some(65));
        octree.set(IndexPoint(-3, 9, -11), Some(65));
        octree.set(IndexPoint(-1, -1, -2), Some(62));
        octree.set(IndexPoint(0, 0, 0), Some(0));

        assert_eq!(octree.count(|v| *v >= 60 && *v < 70), 5);
    }

    #[test]
    fn can_poke_hole_and_count_correctly() {
        // Cube 0,0,0 to 16,16,16
        let mut octy = Octree::<i32> {
            root: Octant::Branch(Box::new([
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(None),
                Octant::Leaf(Some(64)),
            ])),
            factor: 16,
        };

        /*

        Branch([
            Leaf(None),
            Leaf(None),
            Leaf(None),
            Leaf(None),
            Leaf(None),
            Leaf(None),
            Leaf(None),
            Branch([
                Branch([
                    Branch([
                        Leaf(Some(64)),
                        Leaf(Some(64)),
                        Leaf(Some(64)),
                        Leaf(Some(64)),
                        Leaf(Some(64)),
                        Leaf(Some(64)),
                        Branch([
                            Leaf(Some(64)),
                            Leaf(Some(64)),
                            Leaf(Some(64)),
                            Leaf(Some(64)),
                            Leaf(Some(64)),
                            Leaf(None),
                            Leaf(Some(64)),
                            Leaf(Some(64))
                        ]),
                        Leaf(Some(64))
                    ]),
                    Leaf(Some(64)),
                    Leaf(Some(64)),
                    Leaf(Some(64)),
                    Leaf(Some(64)),
                    Leaf(Some(64)),
                    Leaf(Some(64)),
                    Leaf(Some(64))
                ]),
                Leaf(Some(64)),
                Leaf(Some(64)),
                Leaf(Some(64)),
                Leaf(Some(64)),
                Leaf(Some(64)),
                Leaf(Some(64)),
                Leaf(Some(64))
              ])
            ])
        */

        assert_eq!(octy.count(|v| *v == 64), 4096);
        octy.set(IndexPoint(1, 2, 3), None);
        assert_eq!(octy.count(|v| *v == 64), 4095);
        octy.set(IndexPoint(1, 2, 3), None);
        assert_eq!(octy.count(|v| *v == 64), 4095);
        octy.set(IndexPoint(6, 7, 4), None);
        assert_eq!(octy.count(|v| *v == 64), 4094);
    }

    #[test]
    fn can_set_cubes() {
        for x in -8..2 {
            for y in -4..3 {
                for z in -3..1 {
                    let mut ot = Octree::new(16, None);
                    ot.set_cube(
                        IndexCube(
                            IndexPoint(x, y, z),
                            IndexPoint(x + 8, y + 8, z + 8),
                        ),
                        Some(32),
                    );

                    ot.set_cube(
                        IndexCube(
                            IndexPoint(x + 4, y + 4, z + 4),
                            IndexPoint(x + 10, y + 10, z + 10),
                        ),
                        None,
                    );

                    assert_eq!(ot.count(|_| true), 448);
                }
            }
        }
    }
}