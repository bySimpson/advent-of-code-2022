use std::{fs, io};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let input = fs::read_to_string(args.path)?;
    let result_1 = get_position(input.clone(), 4);
    println!("Result 1:\t{}", result_1);

    let result_2 = get_position(input, 14);
    println!("Result 2:\t{}", result_2);
    Ok(())
}

fn get_position(input: String, amount_unique_chars: usize) -> usize {
    let mut result = 0;
    'outer: for i in 0..input.len() - amount_unique_chars {
        let c_slice = &input[i..i + amount_unique_chars];
        let mut found_unique = true;
        'inner: for c_char in c_slice.chars() {
            if c_slice.match_indices(c_char).count() != 1 {
                found_unique = false;
                break 'inner;
            }
        }
        if found_unique {
            result = i + amount_unique_chars;
            break 'outer;
        }
    }
    result
}
