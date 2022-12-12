use std::{fs, io, vec};

use clap::Parser;
use pathfinding::prelude::dijkstra;

use crate::height::{HeightMap, Sorting};

mod height;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_12.txt")]
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

    let mut height_map: Vec<Vec<HeightMap>> = vec![];
    // parse input
    for c_line in input_str.lines() {
        height_map.push(vec![]);
        for c_char in c_line.chars() {
            match c_char {
                'E' => height_map.last_mut().unwrap().push(HeightMap::End),
                'S' => height_map.last_mut().unwrap().push(HeightMap::Start),
                _ => height_map
                    .last_mut()
                    .unwrap()
                    .push(HeightMap::Height(c_char as i32)),
            }
        }
    }

    // find start/end & flatten list for pathfinding
    let mut start_coordinates: Option<(i32, i32)> = None;
    let mut end_coordinates: Option<(i32, i32)> = None;
    for y_coordinate in 0..height_map.len() {
        for x_coordinate in 0..height_map[y_coordinate].len() {
            let c_val = height_map[y_coordinate][x_coordinate];
            match c_val {
                HeightMap::Start => {
                    start_coordinates = Some((x_coordinate as i32, y_coordinate as i32))
                }
                HeightMap::End => {
                    end_coordinates = Some((x_coordinate as i32, y_coordinate as i32))
                }
                _ => (),
            }
            /*if start_coordinates != None && end_coordinates != None {
                break 'outer;
            }*/
        }
    }
    if start_coordinates == None || end_coordinates == None {
        panic!("Could not find start/end coordinates!");
    }
    let mut sorting = Sorting::new(
        height_map.clone(),
        start_coordinates.unwrap(),
        end_coordinates.unwrap(),
    );

    let result = dijkstra(
        &start_coordinates.unwrap(),
        |c| {
            sorting
                .get_successors(*c)
                .iter()
                .map(|s| (s.pos, s.cost))
                .collect::<Vec<_>>()
        },
        |g| *g == end_coordinates.unwrap(),
    );
    //part 1
    println!(
        "Points 1:\t{:?}",
        result.expect("No path found!").0.len() - 1
    );

    // part 2
    let mut starting_points: Vec<(i32, i32)> = vec![start_coordinates.unwrap()];
    for y_coordinate in 0..height_map.len() {
        for x_coordinate in 0..height_map[y_coordinate].len() {
            let c_val = height_map[y_coordinate][x_coordinate];
            match c_val {
                HeightMap::Height(val) => {
                    if val == 'a' as i32 {
                        starting_points.push((x_coordinate as i32, y_coordinate as i32))
                    }
                }
                _ => (),
            }
        }
    }

    let mut steps_required: Vec<i32> = vec![];
    for c_starting_point in starting_points {
        sorting.start_coordinates = c_starting_point;
        let result = dijkstra(
            &sorting.start_coordinates,
            |c| {
                sorting
                    .get_successors(*c)
                    .iter()
                    .map(|s| (s.pos, s.cost))
                    .collect::<Vec<_>>()
            },
            |g| *g == sorting.end_coordinates,
        );
        match result {
            Some(val) => steps_required.push(val.0.len() as i32),
            _ => (),
        }
    }
    println!("Points 2:\t{:?}", steps_required.iter().min().unwrap() - 1);
    Ok(())
}
