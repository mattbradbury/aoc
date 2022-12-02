use std::fmt::Display;

use crate::point::Point;


#[derive(Debug, Default, Copy, Clone)]
pub enum Facing {
    
    East,
    #[default]
    South,
    West,
    North,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Dirs {
    #[default] Straight,
    Left,
    Right,
    Behind
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Turtle {
    pub x: isize,
    pub y: isize,
    pub facing: Facing,
}

impl Into<(isize, isize)> for &Turtle {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Into<(isize, isize)> for Turtle {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Into<Point<isize>> for Turtle {
    fn into(self) -> Point<isize> {
        Point  { x: self.x, y: self.y }
    }
}


impl From<(isize, isize)> for Turtle {
    fn from((x,y): (isize, isize)) -> Self {
        Turtle { x, y, ..Default::default() }
    }
}

impl From<Point<isize>> for Turtle {
    fn from(p: Point<isize>) -> Self {
        Turtle { x: p.x, y: p.y, ..Default::default() }
    }
}

impl Display for Turtle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}) {:?}", self.x, self.y, self.facing)
    }
}

impl Turtle {



    pub fn turn(&mut self, dir: Dirs) {
        use Facing::*;
        self.facing = match (self.facing, dir) {
            (Facing::East, Dirs::Left) => North,
            (Facing::East, Dirs::Right) => South,
            (Facing::East, Dirs::Behind) => West,
            (Facing::South, Dirs::Left) => East,
            (Facing::South, Dirs::Right) => West,
            (Facing::South, Dirs::Behind) => North,
            (Facing::West, Dirs::Left) => South,
            (Facing::West, Dirs::Right) => North,
            (Facing::West, Dirs::Behind) => East,
            (Facing::North, Dirs::Left) => West,
            (Facing::North, Dirs::Right) => East,
            (Facing::North, Dirs::Behind) => South,
            (f,_) => { f }
        }
    }

    pub fn peek(&self, dir: Dirs) -> Point<isize> {
        let (x, y) = (self.x, self.y);
        let xy = match (self.facing, dir) {
            (Facing::East, Dirs::Straight) =>  (x + 1, y + 0),
            (Facing::East, Dirs::Left) =>   (x + 0, y + 1),
            (Facing::East, Dirs::Right) =>  (x + 0, y - 1),
            (Facing::East, Dirs::Behind) => todo!(),
            (Facing::South, Dirs::Straight) => (x + 0, y - 1),
            (Facing::South, Dirs::Left) => (x + 1, y + 0),
            (Facing::South, Dirs::Right) => (x - 1, y + 0),
            (Facing::South, Dirs::Behind) => todo!(),
            (Facing::West, Dirs::Straight) => (x - 1, y + 0),
            (Facing::West, Dirs::Left) => (x + 0, y - 1),
            (Facing::West, Dirs::Right) => (x + 0, y + 1),
            (Facing::West, Dirs::Behind) => todo!(),
            (Facing::North, Dirs::Straight) => (x + 0, y + 1),
            (Facing::North, Dirs::Left) => (x - 1, y + 0),
            (Facing::North, Dirs::Right) => (x + 1, y + 0),
            (Facing::North, Dirs::Behind) => (x + 0, y - 1),
        };
        xy.into()
     
    }

    pub fn advance(&mut self, dist: isize) {
        match self.facing {
            Facing::East => self.x += dist,
            Facing::South => self.y -= dist,
            Facing::West => self.x -= dist,
            Facing::North => self.y += dist,
        }
    }
}
