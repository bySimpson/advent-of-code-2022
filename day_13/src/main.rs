use std::cmp::Ordering;
use std::{fs, io, vec};

use clap::Parser;
use intlist::IntList;
mod intlist;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_13.txt")]
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
    // parse input
    let mut index = 1;
    let mut indexes_smaller: Vec<i32> = vec![];
    for c_comparison in input_str.split("\n\n") {
        let parts = c_comparison.split("\n").collect::<Vec<&str>>();
        let left_str = IntList::parse(parts[0].to_string());
        let right_str = IntList::parse(parts[1].to_string());
        match IntList::compare(&left_str, &right_str) {
            Ordering::Less => indexes_smaller.push(index),
            _ => (),
        }
        index += 1;
    }

    println!("Points 1:\t{}", indexes_smaller.iter().sum::<i32>());

    Ok(())
}
