use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Sand,
    Empty,
    Obstacle,
}

pub struct Grid {
    pub items: Vec<Vec<FieldType>>,
    size_x: i32,
    size_y: i32,
    sand_starting_point: (i32, i32),
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y_coord in 0..self.items[0].len() {
            for x_coord in 450..self.items.len() {
                s.push(match self.items[x_coord as usize][y_coord as usize] {
                    FieldType::Sand => 'o',
                    FieldType::Empty => '.',
                    FieldType::Obstacle => '#',
                });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Grid {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        Self {
            items: vec![vec![FieldType::Empty; size_y as usize]; size_x as usize],
            size_x: size_x,
            size_y: size_y,
            sand_starting_point: (500, 0),
        }
    }

    pub fn update_coord_tuple(&mut self, coord: (i32, i32), field_type: FieldType) -> bool {
        if self.size_x < coord.0 || self.size_y < coord.1 {
            panic!("out of bounds!");
        }
        self.items[coord.0 as usize][coord.1 as usize] = field_type;
        true
    }

    pub fn insert_obstacle(&mut self, from: (i32, i32), to: (i32, i32)) {
        if (from.0 - to.0).abs() != 0 {
            let min = from.0.min(to.0);
            let max = from.0.max(to.0);
            //x
            for i in min..=max {
                self.update_coord_tuple((i, from.1), FieldType::Obstacle);
            }
        }
        if (from.1 - to.1).abs() != 0 {
            let min = from.1.min(to.1);
            let max = from.1.max(to.1);
            //y
            for i in min..=max {
                self.update_coord_tuple((from.0, i), FieldType::Obstacle);
            }
        }
    }

    /// simulates sand; true --> still on screen; false --> in abyss
    pub fn simulate_sand(&mut self) -> bool {
        let mut c_sand_position = self.sand_starting_point;
        for y_coord in self.sand_starting_point.1..self.size_y - 1 {
            c_sand_position = (c_sand_position.0, y_coord);
            let collision_down =
                match self.items[c_sand_position.0 as usize][(c_sand_position.1 + 1) as usize] {
                    FieldType::Obstacle | FieldType::Sand => true,
                    FieldType::Empty => false,
                };
            if collision_down {
                //left
                if c_sand_position.0 != 0 {
                    let obstacle_left_down = self.items[(c_sand_position.0 - 1) as usize]
                        [(c_sand_position.1 + 1) as usize];
                    if obstacle_left_down == FieldType::Empty {
                        c_sand_position = (c_sand_position.0 - 1, c_sand_position.1);
                        continue;
                    }
                }
                //right
                if c_sand_position.0 < self.size_x {
                    let obstacle_right_down = self.items[(c_sand_position.0 + 1) as usize]
                        [(c_sand_position.1 + 1) as usize];
                    if obstacle_right_down == FieldType::Empty {
                        c_sand_position = (c_sand_position.0 + 1, c_sand_position.1);
                        continue;
                    }
                }
                break;
            }
        }
        if c_sand_position.1 < self.size_y {
            if self.items[c_sand_position.0 as usize][(c_sand_position.1 + 1) as usize]
                == FieldType::Empty
            {
                return false;
            }
            self.update_coord_tuple(c_sand_position, FieldType::Sand);
            return true;
        } else {
            return false;
        }
    }
}
