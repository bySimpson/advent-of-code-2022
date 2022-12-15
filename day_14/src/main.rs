use std::{fs, io};

use clap::Parser;

use crate::grid::Grid;

mod grid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_14.txt")]
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

    //get grid size
    let mut max_x = 0;
    let mut max_y = 0;
    for c_line in input_str.split("\n") {
        let coords = c_line
            .split(" -> ")
            .map(|c_coord| {
                let cordinate = c_coord
                    .split(",")
                    .map(|axis| axis.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (cordinate[0], cordinate[1])
            })
            .collect::<Vec<(i32, i32)>>();
        max_x = max_x.max(coords.iter().map(|x| x.0).max().unwrap() + 1);
        max_y = max_y.max(coords.iter().map(|x| x.1).max().unwrap() + 1);
    }

    // parse input
    let mut grid = Grid::new(max_x, max_y);
    for c_line in input_str.split("\n") {
        let coords = c_line
            .split(" -> ")
            .map(|c_coord| {
                let cordinate = c_coord
                    .split(",")
                    .map(|axis| axis.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (cordinate[0], cordinate[1])
            })
            .collect::<Vec<(i32, i32)>>();
        for i in 0..coords.len() - 1 as usize {
            grid.insert_obstacle(coords[i], coords[i + 1]);
        }
    }
    let mut count_sand_corns = 0;
    while grid.simulate_sand() {
        count_sand_corns += 1;
    }
    println!("{}", grid);
    println!("Points 1:\t{}", count_sand_corns);

    println!("Points 2:\t{}", "points_2");

    Ok(())
}
