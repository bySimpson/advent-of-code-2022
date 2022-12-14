use std::{collections::HashMap, fs, io, ops::RangeInclusive};

use clap::Parser;

use crate::grid::{combine_ranges, Sensor};

mod grid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_15.txt")]
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

    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: Vec<(i64, i64)> = vec![];

    for c_line in input_str.split("\n") {
        let replaced_string = c_line
            .replace("Sensor at x=", "")
            .replace(", y=", ",")
            .replace(": closest beacon is at x=", ";");

        let objects = replaced_string
            .split(";")
            .map(|object| {
                let coordinates = object
                    .split(",")
                    .map(|item| item.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                (coordinates[0], coordinates[1])
            })
            .collect::<Vec<_>>();
        let sensor_pos = objects[0];
        let beacon_pos = objects[1];
        let c_sensor = Sensor::new(sensor_pos, beacon_pos);
        sensors.push(c_sensor);
        beacons.push(beacon_pos);
    }

    let line_in_question = 2000000;

    let mut overlaps: HashMap<i64, i64> = HashMap::new();
    for c_sensor in sensors.clone() {
        let c_overlap = c_sensor.get_overlap(line_in_question);
        for c_overlap_item in c_overlap {
            if !beacons.contains(&(c_overlap_item, line_in_question)) {
                overlaps.insert(c_overlap_item, c_overlap_item);
            }
        }
    }
    //part 1
    println!("Points 1:\t{}", overlaps.len());

    //part 2
    let size: i64 = 4000000;
    let mut found = (0, 0);
    'outer: for c_line_nmbr in 0..=size {
        let mut overlaps: Vec<RangeInclusive<i64>> = vec![];
        for c_sensor in sensors.clone() {
            let c_overlap = c_sensor.get_overlap_iterators(c_line_nmbr);
            for c_overlap_item in c_overlap {
                //if !beacons.contains(&(c_overlap_item, c_line_nmbr)) {
                overlaps.push(c_overlap_item);
                //}
            }
        }
        combine_ranges(&mut overlaps);
        if overlaps.len() != 1 {
            for range_nmbr in 0..overlaps.len() {
                if overlaps[range_nmbr].end() + 1 != *overlaps[range_nmbr + 1].start() {
                    found = (overlaps[range_nmbr].end() + 1 as i64, c_line_nmbr);
                    break 'outer;
                }
            }
        }
        /*if overlaps.len() != (size + 1) as usize {
            let start = overlaps.iter().min().unwrap().clone();
            let stop = overlaps.iter().max().unwrap().clone();
            //if start <= 0 && stop >= size {
            println!("{}", overlaps.len());
            for c_x_nmbr in start..=stop {
                if !overlaps.contains(&c_x_nmbr) {
                    found = (c_x_nmbr, c_line_nmbr);
                    break 'outer;
                }
            }
        }*/
        //}
    }
    let points = found.0 * 4_000_000 + found.1;
    println!("Points 2:\t{:?}", points);

    Ok(())
}
