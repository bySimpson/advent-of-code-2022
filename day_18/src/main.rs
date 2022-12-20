use std::{fs, io};

use clap::{command, Parser};

use crate::grid::{Cave, FieldType};
mod grid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_18.txt")]
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
    let mut rocks: Vec<(u32, u32, u32)> = vec![];
    let mut highest_x = 0;
    let mut highest_y = 0;
    let mut highest_z = 0;
    for c_line in input_str.lines() {
        let split = c_line
            .split(",")
            .map(|c_item| c_item.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        highest_x = highest_x.max(split[0] + 1);
        highest_y = highest_y.max(split[1] + 1);
        highest_z = highest_z.max(split[2] + 1);
        rocks.push((split[0], split[1], split[2]));
    }
    let mut cave = Cave::new(highest_x as usize, highest_y as usize, highest_z as usize);
    cave.bulk_update_coordinates(rocks, FieldType::Lava);

    println!("Points 1:\t{}", cave.get_surface_area());
    //part 2
    cave.generate_trapped_air();
    println!(
        "Points 2:\t{}",
        cave.get_surface_area() - cave.get_trapped_air_touching_lava()
    );

    //println!("{}", cave);

    Ok(())
}
