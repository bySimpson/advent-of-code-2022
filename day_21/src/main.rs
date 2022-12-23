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

    let mut monkeys_2 = monkeys.clone();

    //part 1
    while monkeys[&String::from("root")].shout == None {
        let mut new_monkeys = monkeys.clone();
        for (_, c_monkey) in new_monkeys.iter_mut() {
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
    monkeys_2.get_mut(&String::from("root")).unwrap().operation = Operation::Equal;
    let monkeys_2_bak = monkeys_2.clone();
    let mut iteration = 0;
    let name_left = &monkeys_2[&String::from("root")]
        .left_monkey
        .clone()
        .unwrap();
    let name_right = &monkeys_2[&String::from("root")]
        .right_monkey
        .clone()
        .unwrap();
    println!("{:?}", monkeys_2_bak[name_right].shout);
    'outer: loop {
        let mut monkeys_2 = monkeys_2_bak.clone();
        monkeys_2.get_mut(&String::from("humn")).unwrap().shout = Some(iteration);
        while monkeys_2[name_left].shout == None || monkeys_2[name_right].shout == None {
            let mut new_monkeys = monkeys_2.clone();
            'inner: for (_, c_monkey) in new_monkeys.iter_mut() {
                // skip if monkey has no one to shout to
                if c_monkey.operation == Operation::None
                    || c_monkey.left_monkey == None
                    || c_monkey.right_monkey == None
                    || c_monkey.shout != None
                    || c_monkey.operation == Operation::Equal
                {
                    continue 'inner;
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
                            _ => None,
                        }
                    }
                }
            }
            monkeys_2 = new_monkeys;
        }
        println!("{}", name_right);
        println!(
            "{:?} - {:?} - {:?}",
            monkeys_2[name_left].shout,
            monkeys_2[name_right].shout,
            monkeys_2[&String::from("humn")].shout
        );
        if monkeys_2[name_left].shout == monkeys_2[name_right].shout {
            break 'outer;
        }

        iteration += 1;
    }

    //part 2
    println!("Points 2:\t{}", iteration);

    //println!("{}", cave);

    Ok(())
}
