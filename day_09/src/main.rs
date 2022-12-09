use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_09.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    //challenge_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    //part 1
    let mut visited = vec![];
    visited.push((0, 0));

    //part 2
    let mut visited_2 = visited.clone();
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let c_line = line?;
        let command = c_line.split(" ").collect::<Vec<&str>>();
        for _ in 0..command[1].parse::<i32>().unwrap() {
            match command[0] {
                "R" => {
                    knots[0].0 += 1;
                }
                "L" => {
                    knots[0].0 -= 1;
                }
                "U" => {
                    knots[0].1 += 1;
                }
                "D" => {
                    knots[0].1 -= 1;
                }
                _ => {
                    panic!("Invalid command!")
                }
            }

            //part 2
            'inner: for i in 1..10 {
                if (knots[i].0 == knots[i - 1].0 && (knots[i].1 - knots[i - 1].1).abs() == 1)
                    || (knots[i].1 == knots[i - 1].1 && (knots[i].0 - knots[i - 1].0).abs() == 1)
                    || (knots[i].0 - knots[i - 1].0).abs() == 1
                        && (knots[i].1 - knots[i - 1].1).abs() == 1
                    || knots[i].0 == knots[i - 1].0 && knots[i].1 == knots[i - 1].1
                {
                    // don't move!
                    continue 'inner;
                } else {
                    knots[i].0 += get_direction(knots[i - 1].0 - knots[i].0);
                    knots[i].1 += get_direction(knots[i - 1].1 - knots[i].1);
                }
                //part 1
                if !visited.contains(&knots[1]) {
                    visited.push(knots[1].clone());
                }
                //part 2
                if !visited_2.contains(&knots.last().unwrap()) {
                    visited_2.push(knots.last().unwrap().clone());
                }
            }
        }
    }

    println!("Points 1:\t{}", visited.len());
    println!("Points 2:\t{}", visited_2.len());
    Ok(())
}

fn get_direction(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}
