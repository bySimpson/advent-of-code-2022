#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HeightMap {
    Start,
    End,
    Height(i32),
}

impl HeightMap {
    pub fn get_height(&self) -> i32 {
        match self {
            HeightMap::Start => 'a' as i32,
            HeightMap::Height(val) => *val,
            HeightMap::End => 'z' as i32,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Sorting {
    height_map: Vec<Vec<HeightMap>>,
    pub start_coordinates: (i32, i32),
    pub end_coordinates: (i32, i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Successor {
    pub pos: (i32, i32),
    pub cost: i32,
}

impl Sorting {
    pub fn new(
        height_map: Vec<Vec<HeightMap>>,
        start_coordinates: (i32, i32),
        end_coordinates: (i32, i32),
    ) -> Self {
        Self {
            height_map,
            start_coordinates,
            end_coordinates,
        }
    }

    pub fn get_successors(&self, position: (i32, i32)) -> Vec<Successor> {
        let mut output: Vec<Successor> = vec![];
        let current = self.height_map[position.1 as usize][position.0 as usize].get_height();
        // up
        if position.1 != 0 {
            let pos_x = position.0 as usize;
            let pos_y = (position.1 - 1) as usize;
            let val = self.height_map[pos_y][pos_x].get_height();
            if current >= val - 1 {
                /*println!(
                    "[{} {}] allowed [{} {}]",
                    position.0, position.1, pos_x, pos_y
                );*/
                output.push(Successor {
                    pos: (pos_x as i32, pos_y as i32),
                    cost: val,
                });
            };
        }
        // left
        if position.0 != 0 {
            let pos_x = (position.0 - 1) as usize;
            let pos_y = (position.1) as usize;
            let val = self.height_map[pos_y][pos_x].get_height();
            if current >= val - 1 {
                /*println!(
                    "[{} {}] allowed [{} {}]",
                    position.0, position.1, pos_x, pos_y
                );*/
                output.push(Successor {
                    pos: (pos_x as i32, pos_y as i32),
                    cost: val,
                });
            }
        }
        // down
        if position.1 + 1 < self.height_map.len() as i32 {
            let pos_x = position.0 as usize;
            let pos_y = (position.1 + 1) as usize;
            let val = self.height_map[pos_y][pos_x].get_height();
            if current >= val - 1 {
                /*println!(
                    "[{} {}] allowed [{} {}]",
                    position.0, position.1, pos_x, pos_y
                );*/
                output.push(Successor {
                    pos: (pos_x as i32, pos_y as i32),
                    cost: val,
                });
            }
        }
        // right
        if position.0 + 1 < self.height_map[position.1 as usize].len() as i32 {
            let pos_x = (position.0 + 1) as usize;
            let pos_y = (position.1) as usize;
            let val = self.height_map[pos_y][pos_x].get_height();
            if current >= val - 1 {
                /*println!(
                    "[{} {}] allowed [{} {}]",
                    position.0, position.1, pos_x, pos_y
                );*/
                output.push(Successor {
                    pos: (pos_x as i32, pos_y as i32),
                    cost: val,
                });
            }
        }
        output
    }
}
