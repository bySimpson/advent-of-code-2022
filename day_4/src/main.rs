use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use assignment::Assignment;
use clap::Parser;
mod assignment;

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
    let args = Args::parse();
    let mut assignments: Vec<Assignment> = vec![];
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let c_assignment = Assignment::new(line?);
        assignments.push(c_assignment);
    }

    let points = assignments
        .iter()
        .map(|c_assignment| c_assignment.is_contained() as u32)
        .sum::<u32>();

    println!("Points 1:\t{}", points);

    let points_adv = assignments
        .iter()
        .map(|c_assignment| c_assignment.is_contained_at_all() as u32)
        .sum::<u32>();

    println!("Points 2:\t{}", points_adv);
    Ok(())
}
