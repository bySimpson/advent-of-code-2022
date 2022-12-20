#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    Air,
    Rock,
}

pub struct Cave {
    pub grid: Vec<Vec<Vec<FieldType>>>,
}

impl Cave {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            grid: vec![vec![vec![FieldType::Air; z]; y]; x],
        }
    }

    pub fn bulk_update_coordinates(&mut self, coords: Vec<(u32, u32, u32)>, field_type: FieldType) {
        for c_coord in coords {
            self.grid[c_coord.0 as usize][c_coord.1 as usize][c_coord.2 as usize] = field_type;
        }
    }

    pub fn get_surface_area(&self) -> u32 {
        let mut surfaces = 0;
        for x_coord in 0..self.grid.len() {
            for y_coord in 0..self.grid[x_coord].len() {
                for z_coord in 0..self.grid[x_coord][y_coord].len() {
                    let c_val = self.grid[x_coord][y_coord][z_coord];
                    if c_val == FieldType::Rock {
                        let mut sides = vec![];
                        if x_coord < self.grid.len() - 1 {
                            //positive x
                            sides.push(self.grid[x_coord + 1][y_coord][z_coord]);
                        }
                        if x_coord > 0 {
                            //negative x
                            sides.push(self.grid[x_coord - 1][y_coord][z_coord]);
                        }
                        if y_coord < self.grid[x_coord].len() - 1 {
                            //positive y
                            sides.push(self.grid[x_coord][y_coord + 1][z_coord])
                        }
                        if y_coord > 0 {
                            //negative y
                            sides.push(self.grid[x_coord][y_coord - 1][z_coord]);
                        }
                        if z_coord < self.grid[x_coord][y_coord].len() - 1 {
                            //positive z
                            sides.push(self.grid[x_coord][y_coord][z_coord + 1]);
                        }
                        if z_coord > 0 {
                            //negative z
                            sides.push(self.grid[x_coord][y_coord][z_coord - 1]);
                        }
                        for c_side in sides {
                            if c_side == FieldType::Air {
                                surfaces += 1;
                            }
                        }
                        // check for edge cases
                        if x_coord == self.grid.len() - 1 || x_coord == 0 {
                            surfaces += 1;
                        }
                        if y_coord == self.grid[x_coord].len() - 1 || y_coord == 0 {
                            surfaces += 1;
                        }
                        if z_coord == self.grid[x_coord][y_coord].len() - 1 || z_coord == 0 {
                            surfaces += 1;
                        }
                    }
                }
            }
        }
        surfaces
    }
}
