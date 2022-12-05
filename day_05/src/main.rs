use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stacks_challenge_1: Vec<Vec<char>> = vec![];
    let mut stacks_challenge_2: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<Vec<usize>> = vec![];
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut instructions_mode = false;
    for line in reader.lines() {
        let c_line = line?;
        if c_line.is_empty() {
            continue;
        }
        if !instructions_mode {
            // parse containers
            if !c_line.contains("[") {
                instructions_mode = true;
                stacks.iter_mut().for_each(|c_stack| c_stack.reverse());
                stacks_challenge_1 = stacks.clone();
                stacks_challenge_2 = stacks.clone();
                continue;
            }
            let mut stack_number = 0;
            let mut counter = 0;
            while counter < c_line.chars().count() {
                let c_char: char = c_line.chars().collect::<Vec<char>>()[counter + 1];
                if stacks.get(stack_number) == None {
                    stacks.push(vec![]);
                }
                if !c_char.is_whitespace() {
                    stacks[stack_number].push(c_char);
                }
                counter += 4;
                stack_number += 1;
            }
        } else {
            let instruction = c_line
                .clone()
                .replace("move ", "")
                .replace(" from ", ";")
                .replace(" to ", ";");
            let split_instruction: Vec<usize> = instruction
                .split(";")
                .map(|c_instruction| c_instruction.parse::<usize>().unwrap())
                .collect();
            instructions.push(split_instruction.clone());
            for _ in 0..split_instruction[0] {
                let c_item = stacks_challenge_1[split_instruction[1] - 1]
                    .last()
                    .unwrap()
                    .clone();
                stacks_challenge_1[split_instruction[2] - 1].push(c_item);
                stacks_challenge_1[split_instruction[1] - 1].pop().unwrap();
            }
        }
    }
    let mut out_string: String = String::from("");
    stacks_challenge_1
        .iter()
        .for_each(|c_stack| out_string.push(*c_stack.last().unwrap()));
    println!("Points 1:\t{}", out_string);

    for c_instruction in instructions {
        for i in (0..c_instruction[0]).rev() {
            let c_len = stacks_challenge_2[c_instruction[1] - 1].len() - 1;
            let c_item = stacks_challenge_2[c_instruction[1] - 1][c_len - i];
            stacks_challenge_2[c_instruction[2] - 1].push(c_item);
        }
        for _ in 0..c_instruction[0] {
            stacks_challenge_2[c_instruction[1] - 1].pop().unwrap();
        }
    }
    let mut out_string_2: String = String::from("");
    stacks_challenge_2
        .iter()
        .for_each(|c_stack| out_string_2.push(*c_stack.last().unwrap()));
    println!("Points 2:\t{}", out_string_2);
    Ok(())
}
