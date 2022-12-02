use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub enum HexDir {
    NE,
    N,
    NW,
    SW,
    S,
    SE
    
}

impl TryFrom<&str> for HexDir {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use HexDir::*;
        let res = match value.to_ascii_lowercase().as_str() {
            "ne" => NE,
            "nw" => NW,
            "s" => S,
            "sw" => SW,
            "se" => SE,
            "n" => N,
            _ => { return Err(()) }
        };
        Ok(res)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HexTurtle {
    q: isize,
    r: isize,
    s: isize,

}

impl HexTurtle {
    pub fn go(&mut self, dir: HexDir) {
        match dir {
            HexDir::N => { self.q += 1; self.r -= 1 },
            HexDir::NE => { self.q += 1; self.s -= 1 },
            HexDir::SE =>  { self.s -= 1; self.r += 1 },
            HexDir::S => { self.q -= 1; self.r += 1 },
            HexDir::SW => { self.q -= 1; self.s += 1 },
            HexDir::NW =>  { self.s += 1; self.r -= 1 },
        }
    }

    pub fn distance_to(&self, rhs: &HexTurtle) -> isize {
        let diff = self - rhs;
        (diff.q.abs() + diff.r.abs() + diff.s.abs()) / 2
    }
}

impl Sub for &HexTurtle {
    type Output = HexTurtle;

    fn sub(self, rhs: Self) -> Self::Output {
        HexTurtle {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}