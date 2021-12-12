#[derive(Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if y < 0 { return None };
        if x < 0 { return None };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x { return None };
        if y >= max_y { return None };
        return Some(&self.grid[y][x]);
    }    
    
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if y < 0 { return None };
        if x < 0 { return None };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x { return None };
        if y >= max_y { return None };
        return Some(&mut self.grid[y][x]);
    }

    pub fn set(&mut self, x: isize, y:isize, val:T) {
        if y < 0 { return  };
        if x < 0 { return  };
        let x = x as usize;
        let y = y as usize;
        let (max_x, max_y) = self.dimensions();
        if x >= max_x { return  };
        if y >= max_y { return  };
        self.grid[y][x] = val
    }
    
    pub fn get_neighbors(&self, x:isize, y:isize) -> Vec<&T> {
        let mut ret = Vec::new();
        for (i,j) in [(-1,0),(1,0),(0,-1),(0,1)] {
            if let Some(val) = self.get(x+i, y+j) {
                ret.push(val);
            }
        };
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
        return (self.grid[0].len(), self.grid.len())
    }
}
