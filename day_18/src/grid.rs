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
                        let positive_x = self.grid[x_coord + 1][y_coord][z_coord];
                        let negative_x = self.grid[x_coord - 1][y_coord][z_coord];
                        let positive_y = self.grid[x_coord][y_coord + 1][z_coord];
                        let negative_y = self.grid[x_coord][y_coord - 1][z_coord];
                        let positive_z = self.grid[x_coord][y_coord][z_coord + 1];
                        let negative_z = self.grid[x_coord][y_coord][z_coord - 1];
                        let sides = vec![
                            positive_x, negative_x, positive_y, negative_y, positive_z, negative_z,
                        ];
                        for c_side in sides {
                            if c_side == FieldType::Air {
                                surfaces += 1;
                            }
                        }
                    }
                }
            }
        }
        surfaces
    }
}
