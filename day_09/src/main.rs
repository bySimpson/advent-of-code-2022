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
    challenge_1().unwrap();
    Ok(())
}

fn challenge_1() -> io::Result<()> {
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut visited = vec![];
    visited.push(tail_pos);
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
                    head_pos.0 += 1;
                }
                "L" => {
                    head_pos.0 -= 1;
                }
                "U" => {
                    head_pos.1 += 1;
                }
                "D" => {
                    head_pos.1 -= 1;
                }
                _ => {
                    panic!("Invalid command!")
                }
            }
            if (head_pos.0 == tail_pos.0 && (head_pos.1 - tail_pos.1).abs() == 1)
                || (head_pos.1 == tail_pos.1 && (head_pos.0 - tail_pos.0).abs() == 1)
                || (head_pos.0 - tail_pos.0).abs() == 1 && (head_pos.1 - tail_pos.1).abs() == 1
            {
                // don't move!
                continue;
            } else {
                tail_pos.0 += get_direction(head_pos.0 - tail_pos.0);
                tail_pos.1 += get_direction(head_pos.1 - tail_pos.1);
                if !visited.contains(&tail_pos) {
                    visited.push(tail_pos);
                }
            }
        }
    }

    //let visible = grid.len() * grid[0].len() - hidden;

    println!("Points 1:\t{}", visited.len());
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
