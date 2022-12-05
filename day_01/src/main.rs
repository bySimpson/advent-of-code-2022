use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let mut elves: Vec<i32> = vec![];
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut c_calories: i32 = 0;
    for line in reader.lines() {
        let line_str = line?.to_string();
        if line_str.is_empty() {
            elves.push(c_calories);
            c_calories = 0;
            continue;
        }
        c_calories += line_str.parse::<i32>().unwrap();
    }

    println!("Most calories:\t{}", elves.iter().max().unwrap());

    elves.sort();
    elves.reverse();

    println!(
        "Sum of top 3:\t{:?}",
        &elves[0..=2].into_iter().sum::<i32>()
    );

    Ok(())
}
