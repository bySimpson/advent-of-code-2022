use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sensor {
    position: (i64, i64),
    nearest_beacon: (i64, i64),
}

impl Sensor {
    pub fn new(position: (i64, i64), nearest_beacon: (i64, i64)) -> Self {
        Self {
            position,
            nearest_beacon,
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        let diff_x = self.position.0 - self.nearest_beacon.0;
        let diff_y = self.position.1 - self.nearest_beacon.1;
        calculate_manhattan((diff_x, diff_y))
    }

    pub fn distance_from_sensor(&self, line_nr: i64) -> i64 {
        (line_nr - self.position.1).abs()
    }

    pub fn get_overlap(&self, line_in_question: i64) -> Vec<i64> {
        let sensor_distance = self.distance_from_sensor(line_in_question);
        if sensor_distance < self.manhattan_distance() {
            let vertical_distance = self.manhattan_distance() - sensor_distance;
            // in range
            return ((self.position.0 - vertical_distance)..=(self.position.0 + vertical_distance))
                .collect::<Vec<i64>>();
        } else {
            // not in range
            return vec![];
        }
    }

    pub fn get_overlap_iterators(&self, line_in_question: i64) -> Option<RangeInclusive<i64>> {
        let sensor_distance = self.distance_from_sensor(line_in_question);
        if sensor_distance < self.manhattan_distance() {
            let vertical_distance = self.manhattan_distance() - sensor_distance;
            // in range
            return Some(
                (self.position.0 - vertical_distance)..=(self.position.0 + vertical_distance),
            );
        } else {
            // not in range
            return None;
        }
    }
}

pub fn calculate_manhattan(coords: (i64, i64)) -> i64 {
    return coords.0.abs() + coords.1.abs();
}

pub fn combine_ranges(ranges: &mut Vec<RangeInclusive<i64>>) {
    ranges.sort_by_cached_key(|range| *range.start());

    'outer: loop {
        for index in 1..ranges.len() {
            if ranges[index - 1].end() >= ranges[index].start() {
                let end = std::cmp::max(ranges[index - 1].end(), ranges[index].end());
                ranges[index - 1] = (*ranges[index - 1].start())..=(*end);
                ranges.remove(index);

                continue 'outer;
            }
        }

        break;
    }
}
