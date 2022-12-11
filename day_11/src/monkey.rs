#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Operation {
    Plus(u64),
    Multiply(u64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Monkey {
    pub operation: Operation, // 0 -> by itself
    pub items: Vec<u64>,      // u64 --> worry level
    pub divisible_by: u64,
    pub if_true: u64,  // to monkey nr
    pub if_false: u64, // to monkey nr
    pub total_inspected_items: u64,
}

impl Monkey {
    pub fn new(input_line: String) -> Self {
        let split_input = input_line.split("\n").collect::<Vec<&str>>();
        // get Operation
        let operation_multiplier = match split_input[2].split(" ").last().unwrap().parse::<u64>() {
            Ok(val) => val,
            _ => 0,
        };
        let operation = match split_input[2] {
            x if x.contains("*") => Operation::Multiply(operation_multiplier),
            x if x.contains("+") => Operation::Plus(operation_multiplier),
            _ => unreachable!(),
        };

        // get items
        let binding = split_input[1].to_string().replace("  Starting items: ", "");
        let items = binding
            .split(", ")
            .map(|c_item| c_item.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        // divisible
        let divisible_by = split_input[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        // if true
        let if_true = split_input[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        // if false
        let if_false = split_input[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        Self {
            operation,
            items,
            divisible_by,
            if_true,
            if_false,
            total_inspected_items: 0,
        }
    }
}
