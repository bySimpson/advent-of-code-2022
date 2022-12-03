pub struct Rucksack {
    compartment_1: String,
    compartment_2: String,
}

impl Rucksack {
    pub fn new(input_line: String) -> Self {
        let compartment_1 = input_line[0..input_line.len() / 2].to_string();
        let compartment_2 = input_line[input_line.len() / 2..input_line.len()].to_string();
        Self {
            compartment_1,
            compartment_2,
        }
    }

    pub fn get_item_apperaring_in_both_parts(&self) -> char {
        for c_char in self.compartment_1.chars() {
            if self.compartment_2.contains(c_char) {
                return c_char;
            }
        }

        panic!("Co match found!");
    }
}
