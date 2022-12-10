use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

mod commands;

use clap::Parser;

use crate::commands::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_10.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    //challenge_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    let mut commands: Vec<Command> = vec![];
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    // parse input
    for line in reader.lines() {
        let c_line = line?;
        commands.push(Command::new(c_line));
    }
    let mut iteration_1 = 0;
    let mut register_1: i32 = 1;
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strengths: Vec<i32> = vec![];

    // part 1
    for c_cmd in commands.clone() {
        for _ in 0..c_cmd.get_iterations() {
            iteration_1 += 1;
            if interesting_cycles.contains(&iteration_1) {
                signal_strengths.push(iteration_1 * register_1);
            }
        }
        match c_cmd {
            Command::Addx(val) => register_1 += val,
            _ => (),
        }
    }
    println!("Points 1:\t{}", signal_strengths.iter().sum::<i32>());

    // part 2
    let mut iteration_2 = 0;
    let mut register_2: i32 = 1;
    for c_cmd in commands {
        for _ in 0..c_cmd.get_iterations() {
            iteration_2 += 1;
            if (register_2..register_2 + 3).contains(&iteration_2) {
                print!("#");
            } else {
                print!(".");
            }
            if iteration_2 % 40 == 0 {
                iteration_2 = 0;
                print!("\n");
            }
            /*if interesting_cycles.contains(&iteration_1) {
                signal_strengths.push(iteration_1 * register_1);
            }*/
        }
        match c_cmd {
            Command::Addx(val) => register_2 += val,
            _ => (),
        }
    }
    //println!("Points 2:\t{}", "");
    Ok(())
}
