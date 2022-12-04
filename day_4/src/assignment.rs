pub struct Assignment {
    pub assignment_1: Vec<u32>,
    pub assignment_2: Vec<u32>,
}

impl Assignment {
    pub fn new(input_line: String) -> Self {
        let assignments_split = input_line.split(",").collect::<Vec<&str>>();
        let assignment_1 = Assignment::generate_vec(assignments_split[0]);
        let assignment_2 = Assignment::generate_vec(assignments_split[1]);

        Self {
            assignment_1,
            assignment_2,
        }
    }

    pub fn is_contained(&self) -> bool {
        let mut contained = true;
        self.assignment_1.iter().for_each(|c_assignment| {
            if !self.assignment_2.contains(c_assignment) {
                contained = false;
                return;
            }
        });
        if !contained {
            contained = true;
            self.assignment_2.iter().for_each(|c_assignment| {
                if !self.assignment_1.contains(c_assignment) {
                    contained = false;
                    return;
                }
            });
        }
        contained
    }

    pub fn is_contained_at_all(&self) -> bool {
        let mut contained = false;
        self.assignment_1.iter().for_each(|c_assignment| {
            if self.assignment_2.contains(c_assignment) {
                contained = true;
                return;
            }
        });
        if !contained {
            self.assignment_2.iter().for_each(|c_assignment| {
                if self.assignment_1.contains(c_assignment) {
                    contained = true;
                    return;
                }
            });
        }
        contained
    }

    fn generate_vec(range: &str) -> Vec<u32> {
        let from_to: Vec<u32> = range
            .split("-")
            .map(|range_str| range_str.parse::<u32>().unwrap())
            .collect();

        (from_to[0]..=from_to[1]).collect()
    }
}
