use std::{ops::Index, fmt::Display};

use itertools::Itertools;

use crate::point::{BoundedPoint, Point};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    dimensions: (usize, usize)
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let dimensions = (grid[0].len(), grid.len());
        Self { grid, dimensions }
    }
    
    pub fn wrapping_get(&self, mut x: isize, mut y: isize) -> &T {
        while x < 0 { x += self.dimensions.0 as isize }
        while y < 0 { y += self.dimensions.1 as isize }

        while x >= self.dimensions.0 as isize { x -= self.dimensions.0 as isize }
        while y >= self.dimensions.1 as isize { y -= self.dimensions.1 as isize }

        let x = x as usize;
        let y = y as usize;

        &self.grid[y][x]
    }
    

    pub fn wrapping_get_mut(&mut self, mut x: isize, mut y: isize) -> &mut T {
        while x < 0 { x += self.dimensions.0 as isize }
        while y < 0 { y += self.dimensions.1 as isize }

        while x >= self.dimensions.0 as isize { x -= self.dimensions.0 as isize }
        while y >= self.dimensions.1 as isize { y -= self.dimensions.1 as isize }

        let x = x as usize;
        let y = y as usize;

        &mut self.grid[y][x]
    }

    pub fn wrapping_set(&mut self, mut x: isize, mut y: isize, val: T) {
        while x < 0 { x += self.dimensions.0 as isize }
        while y < 0 { y += self.dimensions.1 as isize }

        while x >= self.dimensions.0 as isize { x -= self.dimensions.0 as isize }
        while y >= self.dimensions.1 as isize { y -= self.dimensions.1 as isize }

        let x = x as usize;
        let y = y as usize;

        self.grid[y][x] = val
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if y < 0 {
            return None;
        };
        if x < 0 {
            return None;
        };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x {
            return None;
        };
        if y >= max_y {
            return None;
        };
        return Some(&self.grid[y][x]);
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if y < 0 {
            return None;
        };
        if x < 0 {
            return None;
        };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x {
            return None;
        };
        if y >= max_y {
            return None;
        };
        return Some(&mut self.grid[y][x]);
    }

    pub fn set(&mut self, x: isize, y: isize, val: T) {
        if y < 0 {
            return;
        };
        if x < 0 {
            return;
        };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x {
            return;
        };
        if y >= max_y {
            return;
        };
        self.grid[y][x] = val
    }

    pub fn get_neighbors(&self, x: isize, y: isize) -> Vec<&T> {
        let mut ret = Vec::new();
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(val) = self.get(x + i, y + j) {
                ret.push(val);
            }
        }
        ret
    }

    pub fn get_neighbors_points(&self, x: isize, y: isize) -> Vec<(&T, Point<isize>)> {
        let mut ret = Vec::new();
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(val) = self.get(x + i, y + j) {
                ret.push((val, Point { x: x + i, y: y + j }));
            }
        }
        ret
    }

    // pub fn get_neighbors8_mut(&mut self, x:isize, y:isize) -> Vec<&mut T> {
    //     let mut ret = Vec::new();
    //     for (i,j) in
    //     [
    //     (-1,-1), (0,-1), (1,-1),
    //     (-1,0),          (1,0),
    //     (-1,1),  (0,1),  (1,1)] {
    //         if let Some(val) = self.get_mut(x+i, y+j) {
    //             ret.push(val);
    //         }
    //     };
    //     ret
    // }

    pub fn dimensions(&self) -> (usize, usize) {
        return self.dimensions;
    }

    pub fn dimensions_signed(&self) -> (isize, isize) {
        return (self.dimensions.0 as isize , self.dimensions.1 as isize)
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.grid.iter().map(|row| -> String { row.iter().collect() }).join("\n");
        write!(f, "{}", output)
    }
}

impl<T> Index<Point<isize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point<isize>) -> &Self::Output {
        &self.grid[index.y as usize][index.x as usize]
    }
}

impl<T> Index<&BoundedPoint> for Grid<T> {
    type Output = T;

    fn index(&self, index: &BoundedPoint) -> &Self::Output {
        let p = index.get();
        &self.grid[p.y as usize][p.x as usize]
    }
}

impl<T> Index<BoundedPoint> for Grid<T> {
    type Output = T;

    fn index(&self, index: BoundedPoint) -> &Self::Output {
        let p = index.get();
        &self.grid[p.y as usize][p.x as usize]
    }
}
