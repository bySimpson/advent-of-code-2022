use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_08.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let c_line = line?;
        grid.push(vec![]);
        for c_char in c_line.chars() {
            let c_nmbr = c_char.to_digit(10).unwrap() as i32;
            grid.last_mut().unwrap().push(c_nmbr);
        }
    }

    let mut visible = 0;
    let mut viewing_score: Vec<i32> = vec![];
    for c_pos_vertical in 0..grid.len() {
        //println!("{:?}", grid[c_pos_vertical]);
        for c_pos_horizontal in 0..grid[c_pos_vertical].len() {
            //let vertical_line = get_vertical_line(grid, c_pos_horizontal);
            visible +=
                is_visible(grid.clone(), c_pos_horizontal as i32, c_pos_vertical as i32) as i32;
            viewing_score.push(viewing_distance(
                grid.clone(),
                c_pos_horizontal as i32,
                c_pos_vertical as i32,
            ));
        }
    }

    //let visible = grid.len() * grid[0].len() - hidden;

    println!("Points 1:\t{}", visible);
    println!("Points 2:\t{}", viewing_score.iter().max().unwrap());
    Ok(())
}

fn is_visible(grid: Vec<Vec<i32>>, horizontal_pos: i32, vertical_pos: i32) -> bool {
    if horizontal_pos == 0 || vertical_pos == 0 {
        return true;
    }
    let c_val = grid[vertical_pos as usize][horizontal_pos as usize];
    let vertical_line = get_vertical_line(grid.clone(), horizontal_pos);
    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    let mut visible_from_left = true;
    let mut visible_from_right = true;

    // visible from top
    for c_tree_pos in 0..vertical_pos {
        if vertical_line[c_tree_pos as usize] >= c_val {
            visible_from_top = false;
            break;
        }
    }

    // visible from bottom
    for c_tree_pos in vertical_pos + 1..vertical_line.len() as i32 {
        if vertical_line[c_tree_pos as usize] >= c_val {
            visible_from_bottom = false;
            break;
        }
    }

    // visible from left
    for c_tree_pos in 0..horizontal_pos {
        if grid[vertical_pos as usize][c_tree_pos as usize] as i32 >= c_val {
            visible_from_left = false;
            break;
        }
    }

    // visible from right
    for c_tree_pos in horizontal_pos + 1..grid[vertical_pos as usize].len() as i32 {
        if grid[vertical_pos as usize][c_tree_pos as usize] as i32 >= c_val {
            visible_from_right = false;
            break;
        }
    }
    /*println!(
        "x: {} y: {} -  t: {} b: {} l: {} r: {}",
        vertical_pos,
        horizontal_pos,
        visible_from_top,
        visible_from_bottom,
        visible_from_left,
        visible_from_right
    );*/
    return visible_from_top || visible_from_bottom || visible_from_left || visible_from_right;
}

fn viewing_distance(grid: Vec<Vec<i32>>, horizontal_pos: i32, vertical_pos: i32) -> i32 {
    if horizontal_pos == 0 || vertical_pos == 0 {
        return 0;
    }
    let c_val = grid[vertical_pos as usize][horizontal_pos as usize];
    let vertical_line = get_vertical_line(grid.clone(), horizontal_pos);
    let mut viewing_distance_top = 0;
    let mut viewing_distance_bottom = 0;
    let mut viewing_distance_left = 0;
    let mut viewing_distance_right = 0;

    // visible from top
    for c_tree_pos in (0..vertical_pos).rev() {
        viewing_distance_top += 1;
        if vertical_line[c_tree_pos as usize] >= c_val {
            //viewing_distance_top += 1;
            break;
        }
    }

    // visible from bottom
    for c_tree_pos in vertical_pos + 1..vertical_line.len() as i32 {
        viewing_distance_bottom += 1;
        if vertical_line[c_tree_pos as usize] >= c_val {
            //viewing_distance_bottom += 1;
            break;
        }
    }

    // visible from left
    for c_tree_pos in (0..horizontal_pos).rev() {
        viewing_distance_left += 1;
        if grid[vertical_pos as usize][c_tree_pos as usize] as i32 >= c_val {
            break;
        }
        //viewing_distance_left += 1;
    }

    // visible from right
    for c_tree_pos in horizontal_pos + 1..grid[vertical_pos as usize].len() as i32 {
        viewing_distance_right += 1;
        if grid[vertical_pos as usize][c_tree_pos as usize] as i32 >= c_val {
            //viewing_distance_right += 1;
            break;
        }
    }
    /*println!(
        "v: {} x: {} y: {} -  t: {} b: {} l: {} r: {}",
        c_val,
        vertical_pos,
        horizontal_pos,
        viewing_distance_top,
        viewing_distance_bottom,
        viewing_distance_left,
        viewing_distance_right
    );*/
    return viewing_distance_top
        * viewing_distance_bottom
        * viewing_distance_left
        * viewing_distance_right;
}

fn get_vertical_line(grid: Vec<Vec<i32>>, horizontal_pos: i32) -> Vec<i32> {
    let mut out_vec: Vec<i32> = vec![];
    for c_line in grid {
        let c_val = c_line.get(horizontal_pos as usize).unwrap();
        out_vec.push(c_val.to_owned());
    }
    out_vec
}
