use std::fmt::Display;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl TryFrom<&str> for Point {
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
