#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Operation {
    Plus(i32),
    Multiply(i32),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Monkey {
    pub operation: Operation, // 0 -> by itself
    pub items: Vec<i32>,      // i32 --> worry level
    pub divisible_by: i32,
    pub if_true: i32,  // to monkey nr
    pub if_false: i32, // to monkey nr
    pub total_inspected_items: i32,
}

impl Monkey {
    pub fn new(input_line: String) -> Self {
        let split_input = input_line.split("\n").collect::<Vec<&str>>();
        // get Operation
        let operation_multiplier = match split_input[2].split(" ").last().unwrap().parse::<i32>() {
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
            .map(|c_item| c_item.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // divisible
        let divisible_by = split_input[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // if true
        let if_true = split_input[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // if false
        let if_false = split_input[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
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
