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
        max_y = max_y.max(coords.iter().map(|x| x.1).max().unwrap() + 1);
    }

    // part 2: max + 2 = y
    max_y = max_y + 2;

    // parse input
    let size_x = 1000;
    let mut grid = Grid::new(size_x, max_y);
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
    let mut grid_2 = grid.clone();

    //part 1
    let mut count_sand_corns = 0;
    while grid.simulate_sand() {
        count_sand_corns += 1;
    }
    println!("Points 1:\t{}", count_sand_corns);

    //part 2
    grid_2.insert_obstacle((0, max_y - 1), (size_x - 1, max_y - 1));
    let mut count_sand_corns_2 = 0;
    while grid_2.simulate_sand() {
        count_sand_corns_2 += 1;
    }
    println!("Points 2:\t{}", count_sand_corns_2);

    Ok(())
}
