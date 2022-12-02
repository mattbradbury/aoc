use crate::point::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub lo: Point3<isize>,
    pub hi: Point3<isize>,
}

impl Cuboid {
    // pub fn new(0.0: isize, 1.2: isize, 0.1:isize, 1.1: isize, 0.2: isize, 1.2: isize) -> Self {
    //     Self { 0.0, 1.2, 0.1, 1.1, 0.2, 1.2}
    // }
    pub fn new(lo: Point3<isize>, hi: Point3<isize>) -> Option<Self> {
        if lo.x >= hi.x || lo.y >= hi.y || lo.z >= hi.z {
            return None;
        }
        Some(Self { lo, hi })
    }

    pub fn intersect(&self, other: &Self) -> Option<Cuboid> {
        let xlo = self.lo.0.max(other.lo.0);
        let ylo = self.lo.1.max(other.lo.1);
        let zlo = self.lo.2.max(other.lo.2);

        let xhi = self.hi.0.min(other.hi.0);
        let yhi = self.hi.1.min(other.hi.1);
        let zhi = self.hi.2.min(other.hi.2);

        Cuboid::new(Point3(xlo, ylo, zlo), Point3(xhi, yhi, zhi))
    }

    pub fn contains_xyz(&self, x: isize, y: isize, z: isize) -> bool {
        if !(self.lo.0..=self.hi.0).contains(&x) {
            return false;
        };
        if !(self.lo.1..=self.hi.1).contains(&y) {
            return false;
        };
        if !(self.lo.2..=self.hi.2).contains(&z) {
            return false;
        };
        true
    }

    pub fn contains(&self, p: Point3<isize>) -> bool {
        self.contains_xyz(p.0, p.1, p.2)
    }

    pub fn vertices(&self) -> [Point3<isize>; 8] {
        [
            Point3(self.lo.0, self.lo.1, self.lo.2),
            Point3(self.lo.0, self.lo.1, self.hi.2),
            Point3(self.lo.0, self.hi.1, self.lo.2),
            Point3(self.lo.0, self.hi.1, self.hi.2),
            Point3(self.hi.2, self.lo.1, self.lo.2),
            Point3(self.hi.2, self.lo.1, self.hi.2),
            Point3(self.hi.2, self.hi.1, self.lo.2),
            Point3(self.hi.2, self.hi.1, self.hi.2),
        ]
    }

    pub fn surrounds(&self, other: &Cuboid) -> bool {
        let points = other.vertices();
        for p in points {
            if !self.contains(p) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::{point::Point3, Cuboid};

    #[test]
    fn test_intersects() {
        let p0 = Point3(0, 0, 0);
        let p1 = Point3(1, 1, 1);
        let p2 = Point3(2, 2, 2);
        let p3 = Point3(3, 3, 3);

        let cube0 = Cuboid::new(p0, p1).unwrap();
        let cube1 = Cuboid::new(p1, p3).unwrap();
        // let cube2 = Cuboid::new(10,12, 10, 12, 10, 12);
        let cube3 = Cuboid::new(p2, p3).unwrap();

        assert!(cube0.intersect(&cube1).is_some());
        assert!(cube1.intersect(&cube0).is_some());
        // assert!(!cube0.intersects(&cube2));
        // assert!(!cube2.intersects(&cube0));
        assert!(cube3.intersect(&cube0).is_some());
        assert!(cube0.intersect(&cube3).is_some());
    }

    #[test]
    fn test_contains() {
        let p0 = Point3(0, 0, 0);
        // let p1 = Point3(1,1,1);
        let p2 = Point3(2, 2, 2);
        let cube0 = Cuboid::new(p0, p2).unwrap();
        assert!(cube0.contains_xyz(1, 1, 1));
        assert!(cube0.contains_xyz(0, 1, 0));
        assert!(!cube0.contains_xyz(3, 1, 3));
    }
}
