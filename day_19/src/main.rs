use std::{fs, io};

use clap::{command, Parser};

use crate::factory::{Blueprint, Cost};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_19.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

mod factory;

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // prepare to support both inputs
    let mut input_str = fs::read_to_string(args.path)?.replace("\r\n", "\n");
    if input_str.contains("\n\n") {
        input_str = input_str
            .replace("\t", "")
            .trim()
            .split("\n\n")
            .map(|c_chunk| c_chunk.replace("  ", " ").replace("\n", ""))
            .collect::<Vec<_>>()
            .join("\n");
    }

    let mut blueprints: Vec<Blueprint> = vec![];
    for c_line in input_str.lines() {
        let split_line = c_line
            .replace("Blueprint ", "")
            .replace(": Each ore robot costs ", ";")
            .replace(" ore. Each clay robot costs ", ";")
            .replace(" ore. Each obsidian robot costs ", ";")
            .replace(" clay. Each geode robot costs ", ";")
            .replace(" obsidian.", "")
            .replace(" ore and ", ";")
            .split(";")
            .map(|c_str| c_str.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let cost_ore_robot = Cost::new(split_line[1], 0, 0);
        let cost_clay_robot = Cost::new(split_line[2], 0, 0);
        let cost_obsidian_robot = Cost::new(split_line[3], split_line[4], 0);
        let cost_geode_robot = Cost::new(split_line[5], 0, split_line[6]);
        let c_blueprint = Blueprint::new(
            split_line[0],
            cost_ore_robot,
            cost_clay_robot,
            cost_obsidian_robot,
            cost_geode_robot,
        );
        println!("{:?}", c_blueprint);
        blueprints.push(c_blueprint);
    }

    println!("Points 1:\t{}", "cave.get_surface_area()");
    //part 2
    println!("Points 2:\t{}", "");

    //println!("{}", cave);

    Ok(())
}
