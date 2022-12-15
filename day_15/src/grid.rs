use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sensor {
    position: (i32, i32),
    nearest_beacon: (i32, i32),
}

impl Sensor {
    pub fn new(position: (i32, i32), nearest_beacon: (i32, i32)) -> Self {
        Self {
            position,
            nearest_beacon,
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        let diff_x = self.position.0 - self.nearest_beacon.0;
        let diff_y = self.position.1 - self.nearest_beacon.1;
        calculate_manhattan((diff_x, diff_y))
    }

    pub fn distance_from_sensor(&self, line_nr: i32) -> i32 {
        (line_nr - self.position.1).abs()
    }

    pub fn get_overlap(&self, line_in_question: i32) -> Vec<i32> {
        let sensor_distance = self.distance_from_sensor(line_in_question);
        if sensor_distance < self.manhattan_distance() {
            let vertical_distance = self.manhattan_distance() - sensor_distance;
            // in range
            return ((self.position.0 - vertical_distance)..=(self.position.0 + vertical_distance))
                .collect::<Vec<i32>>();
        } else {
            // not in range
            return vec![];
        }
    }
}

pub fn calculate_manhattan(coords: (i32, i32)) -> i32 {
    return coords.0.abs() + coords.1.abs();
}
