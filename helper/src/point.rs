use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Range},
    str::FromStr, string::ParseError,
};

use itertools::Itertools;
use num::Num;

use crate::{Turtle, error::HelperError};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<isize> {
    pub fn manhattan(&self, rhs: &Point<isize>) -> usize {
        ((self.x as isize - rhs.x as isize).abs() + (self.y as isize - rhs.y as isize).abs())
            as usize
    }

    pub fn neigh9(&self) -> Vec<Point<isize>> {
        (-1..2)
            .map(|j| {
                (-1..2).map(move |i| Point {
                    x: self.x + i,
                    y: self.y + j,
                })
            })
            .flatten()
            .collect_vec()
    }

    pub fn neigh8(&self) -> Vec<Point<isize>> {
        (-1..2)
            .map(|j| {
                (-1..2).filter_map(move |i| { 
                    if i == 0 && j == 0 { None }
                    else {
                        Some(Point {
                        x: self.x + i,
                        y: self.y + j,
                        })
                    }
                })
            })
            .flatten()
            .collect_vec()
    }

    pub fn neigh4(&self) -> Vec<Point<isize>> {
        (-1..2)
            .map(|j| {
                (-1..2).filter_map(move |i| { 
                    if !(i == 0 || j == 0) { None }
                    else {
                        Some(Point {
                        x: self.x + i,
                        y: self.y + j,
                        })
                    }
                })
            })
            .flatten()
            .collect_vec()
    }
}

impl<T> Add for Point<T>
where
    T: Add + Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<T> std::fmt::Debug for Point<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?}", self.x, self.y)
    }
}

impl<T> TryFrom<&str> for Point<T>
where
    T: FromStr,
{
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err("Must be two integers seperated by comma");
        };

        let x = match parts[0].parse() {
            Err(_) => return Err("First value is not a valid integer"),
            Ok(x) => x,
        };
        let y = match parts[1].parse() {
            Err(_) => return Err("Second value is not a valid integer"),
            Ok(y) => y,
        };

        Ok(Point { x, y })
    }
}

impl From<(isize, isize)> for Point<isize> {
    fn from((x,y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

// impl From<Turtle> for Point<isize> {
//     fn from(t: Turtle) -> Self {
//         Point { x: t.x, y: t.y }
//     }
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BoundedPoint {
    point: Point<isize>,
    xr: Range<isize>,
    yr: Range<isize>,
}

impl BoundedPoint {
    pub fn new(xr: Range<isize>, yr: Range<isize>) -> Self {
        Self {
            point: Point {
                x: xr.start,
                y: yr.start,
            },
            xr,
            yr,
        }
    }

    pub fn set(&mut self, p: Point<isize>) {
        self.point = p;
        if p.x < self.xr.start {
            self.point.x = self.xr.start
        }
        if p.x >= self.xr.end {
            self.point.x = self.xr.end - 1
        }
        if p.y < self.yr.start {
            self.point.y = self.yr.start
        }
        if p.y >= self.yr.end {
            self.point.y = self.yr.end - 1
        }
    }

    pub fn get<'a>(&'a self) -> &'a Point<isize> {
        &self.point
    }
}

impl Display for BoundedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point)
    }
}

impl Add<Point<isize>> for BoundedPoint {
    type Output = BoundedPoint;

    fn add(self, rhs: Point<isize>) -> Self::Output {
        let mut ret = (self).clone();
        ret.set(self.point + rhs);
        ret
    }
}

impl AddAssign<Point<isize>> for BoundedPoint {
    fn add_assign(&mut self, rhs: Point<isize>) {
        self.set(self.point + rhs);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point3<N: Num> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Display for Point3<N>
where
    N: Num + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl<N> FromStr for Point3<N> where N: Num + Display + FromStr, HelperError: From<<N as FromStr>::Err> {
    type Err = HelperError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.trim().split(',');
        let x = input.next().ok_or(HelperError::ParseError)?.parse::<N>()?;
        let y = input.next().ok_or(HelperError::ParseError)?.parse::<N>()?;
        let z = input.next().ok_or(HelperError::ParseError)?.parse::<N>()?;
        Ok(Self{x, y, z})
    }
}

impl<N> AddAssign for Point3<N> where N: Num + Display + AddAssign {
    fn add_assign(&mut self, rhs: Self) {
         self.x += rhs.x;
         self.y += rhs.y;
         self.z += rhs.z;
    }
}

impl<N> Add for Point3<N> where N: Num + Display + Add {
    type Output = Point3<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }

    }
}

