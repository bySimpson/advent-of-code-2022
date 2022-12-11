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
    let monkeys_2 = monkeys_1.clone();

    //part 1
    println!("Points 1:\t{:?}", simulate_rounds(monkeys_1, 20, |x| x / 3));

    // part 2
    let base: u64 = monkeys_2.iter().map(|x| x.divisible_by).product();
    println!(
        "Points 2:\t{:?}",
        simulate_rounds(monkeys_2, 10000, |x| x % base)
    );
    Ok(())
}

fn simulate_rounds(mut monkeys: Vec<Monkey>, rounds: usize, func: impl Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
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
                // apply base
                c_item = func(c_item);
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
        .collect::<Vec<u64>>();
    monkey_interactions.sort();
    monkey_interactions.iter().rev().take(2).product::<u64>()
}
