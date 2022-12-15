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

    let mut integerlists: Vec<IntList> = vec![];
    for c_comparison in input_str.split("\n\n") {
        let parts = c_comparison.split("\n").collect::<Vec<&str>>();
        let left = IntList::parse(parts[0].to_string());
        let right = IntList::parse(parts[1].to_string());
        match IntList::compare(&left, &right) {
            Ordering::Less => indexes_smaller.push(index),
            _ => (),
        }

        integerlists.push(left);
        integerlists.push(right);
        index += 1;
    }

    println!("Points 1:\t{}", indexes_smaller.iter().sum::<i32>());

    //part 2
    let seperators = vec![
        IntList::List(vec![IntList::List(vec![IntList::Integer(2)])]),
        IntList::List(vec![IntList::List(vec![IntList::Integer(6)])]),
    ];
    integerlists.append(&mut seperators.clone());
    integerlists.sort_by(IntList::compare);
    let mut points_2 = 1;
    for i in 0..integerlists.len() {
        if seperators.contains(&integerlists[i]) {
            points_2 *= i + 1;
        }
    }

    println!("Points 2:\t{}", points_2);

    Ok(())
}
