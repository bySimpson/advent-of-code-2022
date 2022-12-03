use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use rucksack::Rucksack;

mod rucksack;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1().unwrap();
    challenge_2().unwrap();
    Ok(())
}

fn challenge_1() -> io::Result<()> {
    let mut rucksäcke: Vec<Rucksack> = vec![];
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let rucksack = Rucksack::new(line?);
        rucksäcke.push(rucksack);
    }

    let points: u32 = rucksäcke
        .iter()
        .map(|c_rucksack| get_points(c_rucksack.get_item_apperaring_in_both_parts()))
        .sum();

    println!("Points 1:\t{}", points);
    Ok(())
}

fn get_points(c: char) -> u32 {
    match c as u32 {
        n @ 65..=90 => {
            //uppercase
            n - 38
        }
        n @ 97..=122 => {
            //lowercase
            n - 96
        }
        _ => panic!("Invalid input!"),
    }
}

fn challenge_2() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut counter = 0;
    let mut points = 0;
    while counter < lines.len() {
        'inner: for c_char in lines[counter].chars() {
            if lines[counter + 1].contains(c_char) && lines[counter + 2].contains(c_char) {
                points += get_points(c_char);
                break 'inner;
            }
        }

        counter += 3;
    }
    println!("Points 2:\t{}", points);

    Ok(())
}
