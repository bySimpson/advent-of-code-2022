#[derive(Debug, Clone, Copy)]
pub enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    pub fn get_iterations(&self) -> i32 {
        match &self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }

    pub fn new(input: String) -> Self {
        let input_split = input.split(" ").collect::<Vec<&str>>();
        match input_split[0] {
            "addx" => Command::Addx(input_split[1].parse::<i32>().unwrap()),
            "noop" => Command::Noop,
            _ => unreachable!(),
        }
    }
}
