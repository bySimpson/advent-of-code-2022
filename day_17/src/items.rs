use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u64)]
pub enum Shape {
    HorizontalLine = 0,
    Cross = 1,
    L = 2,
    VerticalLine = 3,
    Cube = 4,
}

impl Shape {
    pub fn get_rock_formation(&self, starting_position: (u64, u64)) -> Vec<(u64, u64)> {
        let output_vec: Vec<(u64, u64)> = match self {
            Self::HorizontalLine => {
                vec![
                    (starting_position.0 + 2, starting_position.1), // ..####.
                    (starting_position.0 + 3, starting_position.1),
                    (starting_position.0 + 4, starting_position.1),
                    (starting_position.0 + 5, starting_position.1),
                ]
            }
            Self::Cross => {
                vec![
                    (starting_position.0 + 3, starting_position.1),
                    (starting_position.0 + 2, starting_position.1 + 1), //...#...
                    (starting_position.0 + 3, starting_position.1 + 1), //..###..
                    (starting_position.0 + 4, starting_position.1 + 1), //...#...
                    (starting_position.0 + 3, starting_position.1 + 2),
                ]
            }
            Self::L => {
                vec![
                    (starting_position.0 + 2, starting_position.1), // ..####.
                    (starting_position.0 + 3, starting_position.1),
                    (starting_position.0 + 4, starting_position.1),
                    (starting_position.0 + 4, starting_position.1 + 1),
                    (starting_position.0 + 4, starting_position.1 + 2),
                ]
            }
            Self::VerticalLine => {
                vec![
                    (starting_position.0 + 2, starting_position.1), //     ..#....
                    (starting_position.0 + 2, starting_position.1 + 1), // ..#....
                    (starting_position.0 + 2, starting_position.1 + 2), // ..#....
                    (starting_position.0 + 2, starting_position.1 + 3), // ..#....
                ]
            }
            Self::Cube => {
                vec![
                    (starting_position.0 + 2, starting_position.1), // ..##...
                    (starting_position.0 + 3, starting_position.1), // ..##...
                    (starting_position.0 + 2, starting_position.1 + 1),
                    (starting_position.0 + 3, starting_position.1 + 1),
                ]
            }
        };
        output_vec
    }

    pub fn get_max_height(&self) -> u64 {
        match &self {
            Self::HorizontalLine => 1,
            Self::Cross => 3,
            Self::L => 3,
            Self::VerticalLine => 4,
            Self::Cube => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rock {
    points: Vec<(u64, u64)>,
    pub shape: Shape,
}

impl Rock {
    pub fn new(shape: Shape, starting_position: (u64, u64)) -> Self {
        Self {
            shape,
            points: shape.get_rock_formation(starting_position),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldType {
    Rock,
    Air,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub grid: Vec<VecDeque<FieldType>>,
    pub current_rock: Rock,
    instructions: Vec<Move>,
    pub total_max_height: u64,
    pub current_max_height: u64,
    pub current_instruction_position: u64,
}

impl Game {
    pub fn new(instructions: Vec<Move>) -> Self {
        Self {
            grid: vec![VecDeque::with_capacity(1100); 7],
            current_rock: Rock::new(Shape::HorizontalLine, (0, 3)),
            instructions,
            total_max_height: 0,
            current_max_height: 0,
            current_instruction_position: 0,
        }
    }

    fn fill_grid_to_height(&mut self, height: u64) {
        if (self.grid[0].len() as u64) < height {
            let amount_to_insert = height - self.grid[0].len() as u64;
            for _ in 0..=amount_to_insert {
                for x_coordinate in 0..self.grid.len() {
                    self.grid[x_coordinate].push_back(FieldType::Air);
                }
            }
        }
    }

    fn update_vec_of_fields(&mut self, field_type: FieldType, items: &Vec<(u64, u64)>) {
        for c_item in items {
            self.grid[c_item.0 as usize][c_item.1 as usize] = field_type;
        }
    }

    fn add_rock_to_grid(&mut self) {
        for c_item in self.current_rock.points.iter() {
            self.grid[c_item.0 as usize][c_item.1 as usize] = FieldType::Rock;
        }
    }

    fn update_height_rock(&mut self, corrigate: bool) {
        let old_max_height = self.current_max_height;

        let mut max_height_of_item = 0;
        for c_item in self.current_rock.points.iter() {
            if corrigate {
                max_height_of_item = max_height_of_item.max(c_item.1);
            } else {
                max_height_of_item = max_height_of_item.max(c_item.1 + 1);
            }
        }
        /*println!(
            "Max height: {}",
            max_height_of_item.max(self.max_height) + 1
        );*/
        self.current_max_height = max_height_of_item.max(old_max_height);
        //self.total_max_height = max_height_of_item.max(self.total_max_height);
        if self.current_max_height - old_max_height > 0 {
            self.total_max_height += self.current_max_height - old_max_height;
        }
    }

    fn update_max_height(&mut self, items: &Vec<(u64, u64)>, corrigate: bool) {
        let old_max_height = self.current_max_height;

        let mut max_height_of_item = 0;
        for c_item in items {
            if corrigate {
                max_height_of_item = max_height_of_item.max(c_item.1);
            } else {
                max_height_of_item = max_height_of_item.max(c_item.1 + 1);
            }
        }
        /*println!(
            "Max height: {}",
            max_height_of_item.max(self.max_height) + 1
        );*/
        self.current_max_height = max_height_of_item.max(old_max_height);
        //self.total_max_height = max_height_of_item.max(self.total_max_height);
        if self.current_max_height - old_max_height > 0 {
            self.total_max_height += self.current_max_height - old_max_height;
        }
    }

    pub fn move_rock(&mut self) {
        let shape_height = self.current_rock.shape.get_max_height();
        self.fill_grid_to_height(shape_height + self.current_max_height + 3);

        //move pieces
        'outer: loop {
            let c_instruction = self.instructions[self.current_instruction_position as usize];
            if (self.current_instruction_position as usize) < self.instructions.len() - 1 {
                self.current_instruction_position += 1;
            } else {
                self.current_instruction_position = 0;
            }
            //left/right movement
            /*println!(
                "Shape: {:?}, Move: {:?}",
                self.current_rock.shape, c_instruction
            );*/
            let movable: bool = match c_instruction {
                Move::Right => {
                    let mut movable = true;
                    'inner: for c_point in self.current_rock.points.iter() {
                        if c_point.0 + 1 >= 7
                            || self.grid[(c_point.0 + 1) as usize][c_point.1 as usize]
                                != FieldType::Air
                        {
                            //c_point.0 += 1;
                            movable = false;
                            break 'inner;
                        }
                    }
                    movable
                }
                Move::Left => {
                    let mut movable = true;
                    'inner: for c_point in self.current_rock.points.iter() {
                        if c_point.0 <= 0
                            || self.grid[(c_point.0 - 1) as usize][c_point.1 as usize]
                                != FieldType::Air
                        {
                            //c_point.0 -= 1;
                            movable = false;
                            break 'inner;
                        }
                    }
                    movable
                }
            };
            if movable {
                match c_instruction {
                    Move::Left => {
                        for c_point in self.current_rock.points.iter_mut() {
                            c_point.0 -= 1;
                        }
                    }
                    Move::Right => {
                        for c_point in self.current_rock.points.iter_mut() {
                            c_point.0 += 1;
                        }
                    }
                }
            }

            //down movement
            for c_point in self.current_rock.points.iter() {
                if c_point.1 == 0
                    || self.grid[c_point.0 as usize][(c_point.1 - 1) as usize] != FieldType::Air
                {
                    // stop movement if down movement is no longer possible!
                    break 'outer;
                }
            }

            for c_point in self.current_rock.points.iter_mut() {
                c_point.1 -= 1;
            }
        }
        //let fields_to_update = self.current_rock.points.clone();
        //self.update_vec_of_fields(FieldType::Rock, &fields_to_update);
        self.add_rock_to_grid();

        //optimize parsing
        if self.grid[0].len() > 1000 {
            for x_coordinate in 0..self.grid.len() {
                self.grid[x_coordinate].pop_front();
            }
            self.current_max_height -= 1;
            //self.update_max_height(&fields_to_update, true);
            self.update_height_rock(true);
        } else {
            //self.update_max_height(&fields_to_update, false);
            self.update_height_rock(false);
        }
    }

    pub fn prepare_next_rock(&mut self) {
        //prepare next rock
        let next_rock_shape = match self.current_rock.shape {
            Shape::HorizontalLine => Shape::Cross,
            Shape::Cross => Shape::L,
            Shape::L => Shape::VerticalLine,
            Shape::VerticalLine => Shape::Cube,
            Shape::Cube => Shape::HorizontalLine,
        };
        self.current_rock = Rock::new(next_rock_shape, (0, self.current_max_height + 3));
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y_coord in (0..self.grid[0].len()).rev() {
            s.push('|');
            for x_coord in 0..self.grid.len() {
                //self.items.len() {
                s.push(match self.grid[x_coord as usize][y_coord as usize] {
                    FieldType::Rock => '#',
                    FieldType::Air => '.',
                });
            }
            s.push('|');
            s.push('\n');
        }
        s.push_str("+-------+");
        write!(f, "{}", s)
    }
}
