use core::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    Air,
    Lava,
    TrappedAir,
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

    fn get_neighbours(&self, coord: (usize, usize, usize)) -> Vec<FieldType> {
        let mut sides = vec![];
        let (x_coord, y_coord, z_coord) = coord;
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
        sides
    }

    pub fn get_surface_area(&self) -> u32 {
        let mut surfaces = 0;
        for x_coord in 0..self.grid.len() {
            for y_coord in 0..self.grid[x_coord].len() {
                for z_coord in 0..self.grid[x_coord][y_coord].len() {
                    let c_val = self.grid[x_coord][y_coord][z_coord];
                    if c_val == FieldType::Lava {
                        let sides = self.get_neighbours((x_coord, y_coord, z_coord));
                        for c_side in sides {
                            if c_side != FieldType::Lava {
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

    pub fn get_trapped_air_touching_lava(&self) -> u32 {
        let mut surfaces = 0;
        for x_coord in 0..self.grid.len() {
            for y_coord in 0..self.grid[x_coord].len() {
                'z_loop: for z_coord in 0..self.grid[x_coord][y_coord].len() {
                    let c_val = self.grid[x_coord][y_coord][z_coord];
                    if c_val == FieldType::TrappedAir {
                        let sides = self.get_neighbours((x_coord, y_coord, z_coord));
                        for c_side in sides.iter() {
                            // Trapped air cannot have normal air next to it (Skip flukes!)!
                            if *c_side == FieldType::Air {
                                break 'z_loop;
                            }
                        }
                        for c_side in sides.iter() {
                            if *c_side == FieldType::Lava {
                                surfaces += 1;
                            }
                        }
                    }
                }
            }
        }
        surfaces
    }

    pub fn generate_trapped_air(&mut self) {
        for x_coord in 0..self.grid.len() {
            for y_coord in 0..self.grid[x_coord].len() {
                for z_coord in 0..self.grid[x_coord][y_coord].len() {
                    let c_val = self.grid[x_coord][y_coord][z_coord];
                    if c_val == FieldType::Air {
                        // check x negative
                        let mut trapped_x_negative = false;
                        for current_x in 0..x_coord {
                            let current_to_check = self.grid[current_x][y_coord][z_coord];
                            if current_to_check == FieldType::Lava {
                                trapped_x_negative = true;
                                break;
                            }
                        }
                        // check x positive
                        let mut trapped_x_positive = false;
                        for current_x in x_coord..self.grid.len() {
                            let current_to_check = self.grid[current_x][y_coord][z_coord];
                            if current_to_check == FieldType::Lava {
                                trapped_x_positive = true;
                                break;
                            }
                        }
                        // check y negative
                        let mut trapped_y_negative = false;
                        for current_y in 0..y_coord {
                            let current_to_check = self.grid[x_coord][current_y][z_coord];
                            if current_to_check == FieldType::Lava {
                                trapped_y_negative = true;
                                break;
                            }
                        }
                        // check y positive
                        let mut trapped_y_positive = false;
                        for current_y in y_coord..self.grid[x_coord].len() {
                            let current_to_check = self.grid[x_coord][current_y][z_coord];
                            if current_to_check == FieldType::Lava {
                                trapped_y_positive = true;
                                break;
                            }
                        }
                        // check z negative
                        let mut trapped_z_negative = false;
                        for current_z in 0..z_coord {
                            let current_to_check = self.grid[x_coord][y_coord][current_z];
                            if current_to_check == FieldType::Lava {
                                trapped_z_negative = true;
                                break;
                            }
                        }
                        // check z positive
                        let mut trapped_z_positive = false;
                        for current_z in z_coord..self.grid[x_coord][y_coord].len() {
                            let current_to_check = self.grid[x_coord][y_coord][current_z];
                            if current_to_check == FieldType::Lava {
                                trapped_z_positive = true;
                                break;
                            }
                        }

                        if trapped_x_negative
                            && trapped_x_positive
                            && trapped_y_negative
                            && trapped_y_positive
                            && trapped_z_negative
                            && trapped_z_positive
                        {
                            self.grid[x_coord][y_coord][z_coord] = FieldType::TrappedAir;
                            //println!("Trapped! {} {} {}", x_coord, y_coord, z_coord);
                        }
                    }
                }
            }
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for z_coord in 0..self.grid[0][0].len() {
            s.push('+');
            s.push_str(&"-".repeat(self.grid[0][0].len()));
            s.push('+');
            s.push('\n');
            for y_coord in 0..self.grid[0].len() {
                s.push('|');
                for x_coord in 0..self.grid.len() {
                    //self.items.len() {
                    s.push(
                        match self.grid[x_coord as usize][y_coord as usize][z_coord as usize] {
                            FieldType::Air => ' ',
                            FieldType::TrappedAir => '.',
                            FieldType::Lava => '#',
                        },
                    );
                }
                s.push('|');
                s.push('\n');
            }
            s.push('+');
            s.push_str(&"-".repeat(self.grid[0][0].len()));
            s.push('+');
            s.push('\n');
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
