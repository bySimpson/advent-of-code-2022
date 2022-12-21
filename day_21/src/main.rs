use std::{collections::HashMap, fs, io};

use clap::{command, Parser};

use crate::monkey::{Monkey, Operation};

mod monkey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_21.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // prepare to support both inputs
    let input_str = fs::read_to_string(args.path)?.replace("\r\n", "\n");
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for c_line in input_str.lines() {
        let c_monkey = Monkey::new(c_line);
        monkeys.insert(c_monkey.name.clone(), c_monkey);
    }

    while monkeys[&String::from("root")].shout == None {
        let mut new_monkeys = monkeys.clone();
        for (c_name, c_monkey) in new_monkeys.iter_mut() {
            // skip if monkey has no one to shout to
            if c_monkey.operation == Operation::None
                || c_monkey.left_monkey == None
                || c_monkey.right_monkey == None
                || c_monkey.shout != None
            {
                continue;
            }
            let c_left_monkey = c_monkey.left_monkey.clone().unwrap();
            let c_right_monkey = c_monkey.right_monkey.clone().unwrap();
            let val_left = monkeys.get(&c_left_monkey).unwrap().shout;
            let val_right = monkeys.get(&c_right_monkey).unwrap().shout;
            if let Some(val_left) = val_left {
                if let Some(val_right) = val_right {
                    c_monkey.shout = match c_monkey.operation {
                        Operation::Plus => Some(val_left + val_right),
                        Operation::Minus => Some(val_left - val_right),
                        Operation::Multiply => Some(val_left * val_right),
                        Operation::Divide => Some(val_left / val_right),
                        _ => unreachable!(),
                    }
                }
            }
        }
        monkeys = new_monkeys;
    }

    println!(
        "Points 1:\t{}",
        monkeys[&String::from("root")].shout.unwrap()
    );
    //part 2
    println!("Points 2:\t{}", "");

    //println!("{}", cave);

    Ok(())
}
