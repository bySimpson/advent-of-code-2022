#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    None,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monkey {
    pub name: String,
    pub operation: Operation,
    pub shout: Option<u64>,
    pub left_monkey: Option<String>,
    pub right_monkey: Option<String>,
}

impl Monkey {
    pub fn new(input_line: &str) -> Self {
        let binding = input_line.to_string().replace(": ", ";").replace(" ", "");
        let split = binding.split(";").collect::<Vec<_>>();
        let mut left_monkey = None;
        let mut right_monkey = None;
        let mut shout = None;
        let mut operation = Operation::None;

        // parse Operation
        if split[1].contains("+") {
            operation = Operation::Plus;
            let c_split = split[1].split("+").collect::<Vec<_>>();
            left_monkey = Some(c_split[0].to_string());
            right_monkey = Some(c_split[1].to_string());
        } else if split[1].contains("-") {
            operation = Operation::Minus;
            let c_split = split[1].split("-").collect::<Vec<_>>();
            left_monkey = Some(c_split[0].to_string());
            right_monkey = Some(c_split[1].to_string());
        } else if split[1].contains("/") {
            operation = Operation::Divide;
            let c_split = split[1].split("/").collect::<Vec<_>>();
            left_monkey = Some(c_split[0].to_string());
            right_monkey = Some(c_split[1].to_string());
        } else if split[1].contains("*") {
            operation = Operation::Multiply;
            let c_split = split[1].split("*").collect::<Vec<_>>();
            left_monkey = Some(c_split[0].to_string());
            right_monkey = Some(c_split[1].to_string());
        } else {
            shout = Some(split[1].parse::<u64>().unwrap());
        }

        Self {
            name: split[0].to_string(),
            left_monkey,
            right_monkey,
            shout,
            operation,
        }
    }
}
