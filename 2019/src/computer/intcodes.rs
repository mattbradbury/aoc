use std::{str::FromStr, num::ParseIntError, ops::{Deref, Index, IndexMut}};


#[derive(Debug, Clone)]
pub struct Intcodes(Vec<isize>);

impl FromStr for  Intcodes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split(',').map(|v| v.parse()).collect::<Result<Vec<_>,_>>() {
            Ok(res) => Ok(Intcodes(res)),
            Err(e) => Err(e),
        }
    }
}

impl Deref for Intcodes {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<usize> for Intcodes {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Intcodes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, num::IntErrorKind};

    use crate::computer::Intcodes;


    const EXAMPLE1:&str = "1,0,0,0,99";
    const EXAMPLE1_ARR:[isize; 5] = [1,0,0,0,99];

    const EXAMPLE2:&str = "1,0,0,0,99r";


    #[test]
    fn test_from_str() {
        assert_eq!(Intcodes::from_str(EXAMPLE1).unwrap().0, EXAMPLE1_ARR)
    }

    #[test]
    fn test_from_str_malformed() {
        assert!(Intcodes::from_str(EXAMPLE2).is_err())
    }

    #[test]
    fn test_from_str_empty() {
        assert!(*Intcodes::from_str("").expect_err("").kind() == IntErrorKind::Empty)
    }


}