use std::{fs, io};

use clap::Parser;

use crate::monkey::{Monkey, Operation};
mod monkey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_11.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let input_str = fs::read_to_string(args.path)?.replace("\r\n", "\n");

    let mut monkeys: Vec<Monkey> = vec![];

    // parse input
    for c_monkey_input in input_str.split("\n\n") {
        monkeys.push(Monkey::new(c_monkey_input.to_string()));
    }

    for _ in 0..20 {
        for monkey_nmbr in 0..monkeys.len() {
            for mut c_item in monkeys[monkey_nmbr].items.clone().into_iter() {
                // do inspection
                match monkeys[monkey_nmbr].operation {
                    Operation::Multiply(nmbr) => {
                        if nmbr == 0 {
                            c_item *= c_item;
                        } else {
                            c_item *= nmbr;
                        }
                    }
                    Operation::Plus(nmbr) => {
                        if nmbr == 0 {
                            c_item += c_item;
                        } else {
                            c_item += nmbr;
                        }
                    }
                }
                //less worry after inspection
                c_item /= 3;
                let monkey_nmbr_to_throw = match c_item % monkeys[monkey_nmbr].divisible_by == 0 {
                    true => monkeys[monkey_nmbr].if_true,
                    false => monkeys[monkey_nmbr].if_false,
                };
                monkeys[monkey_nmbr_to_throw as usize].items.push(c_item);

                //increase inspection counter
                monkeys[monkey_nmbr as usize].total_inspected_items += 1;
            }
            monkeys[monkey_nmbr].items = vec![];
        }
    }
    let mut monkey_interactions = monkeys
        .iter()
        .map(|c_monkey| c_monkey.total_inspected_items)
        .collect::<Vec<i32>>();
    monkey_interactions.sort();
    println!(
        "Points 1:\t{:?}",
        monkey_interactions[monkey_interactions.len() - 1]
            * monkey_interactions[monkey_interactions.len() - 2]
    );
    println!("Points 2:\t{}", "");
    Ok(())
}
