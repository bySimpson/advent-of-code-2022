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

    let mut monkeys_1: Vec<Monkey> = vec![];

    // parse input
    for c_monkey_input in input_str.split("\n\n") {
        monkeys_1.push(Monkey::new(c_monkey_input.to_string()));
    }
    let mut monkeys_2 = monkeys_1.clone();

    for _ in 0..20 {
        for monkey_nmbr in 0..monkeys_1.len() {
            for mut c_item in monkeys_1[monkey_nmbr].items.clone().into_iter() {
                // do inspection
                match monkeys_1[monkey_nmbr].operation {
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
                let monkey_nmbr_to_throw = match c_item % monkeys_1[monkey_nmbr].divisible_by == 0 {
                    true => monkeys_1[monkey_nmbr].if_true,
                    false => monkeys_1[monkey_nmbr].if_false,
                };
                monkeys_1[monkey_nmbr_to_throw as usize].items.push(c_item);

                //increase inspection counterbig
                monkeys_1[monkey_nmbr as usize].total_inspected_items += 1;
            }
            monkeys_1[monkey_nmbr].items = vec![];
        }
    }
    let mut monkey_interactions = monkeys_1
        .iter()
        .map(|c_monkey| c_monkey.total_inspected_items)
        .collect::<Vec<i64>>();
    monkey_interactions.sort();
    println!(
        "Points 1:\t{:?}",
        monkey_interactions[monkey_interactions.len() - 1]
            * monkey_interactions[monkey_interactions.len() - 2]
    );

    // get base of divisibles
    let base: i64 = monkeys_2.iter().map(|x| x.divisible_by).product();
    for _ in 0..10000 {
        for monkey_nmbr in 0..monkeys_2.len() {
            for mut c_item in monkeys_2[monkey_nmbr].items.clone().into_iter() {
                // do inspection
                // apply base
                c_item %= base;
                match monkeys_2[monkey_nmbr].operation {
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
                let monkey_nmbr_to_throw = match c_item % monkeys_2[monkey_nmbr].divisible_by == 0 {
                    true => monkeys_2[monkey_nmbr].if_true,
                    false => monkeys_2[monkey_nmbr].if_false,
                };
                monkeys_2[monkey_nmbr_to_throw as usize].items.push(c_item);

                //increase inspection counter
                monkeys_2[monkey_nmbr as usize].total_inspected_items += 1;
            }
            monkeys_2[monkey_nmbr].items = vec![];
        }
    }

    let mut monkey_interactions = monkeys_2
        .iter()
        .map(|c_monkey| c_monkey.total_inspected_items)
        .collect::<Vec<i64>>();
    monkey_interactions.sort();
    println!(
        "Points 2:\t{:?}",
        monkey_interactions[monkey_interactions.len() - 1]
            * monkey_interactions[monkey_interactions.len() - 2]
    );
    Ok(())
}
