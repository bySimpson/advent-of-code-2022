use std::{collections::VecDeque, fs, io};

use clap::{command, Parser};

use crate::items::{FieldType, Game, Move, Shape};

mod items;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_17.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let mut moves: Vec<Move> = vec![];
    let input_str = fs::read_to_string(args.path)?;
    println!("Chars: {}", input_str.chars().count());
    for c_char in input_str.chars() {
        let c_move: Move = match c_char {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => unreachable!("Invalid move!"),
        };
        moves.push(c_move);
    }
    let mut game = Game::new(moves);
    let mut game_2 = game.clone();
    for _ in 0..2022u64 {
        game.move_rock();
        game.prepare_next_rock();
    }

    println!("Points 1:\t{}", game.total_max_height);

    //let mut first_5_lines: Vec<VecDeque<FieldType>> = vec![VecDeque::new(); 7];
    let mut already_occured: Vec<(u64, Shape)> = vec![]; // current height
    let mut height_map: Vec<(u64, u64)> = vec![]; // current heigt, diff to last
    let mut points = 0;
    let rounds = 1000000000000u64;

    for i in 0..rounds {
        let last_height = game_2.total_max_height;
        game_2.move_rock();
        let current = (
            game_2.current_instruction_position,
            game_2.current_rock.shape,
        );
        if already_occured.contains(&current) {
            let height_diff = game_2.total_max_height - last_height;
            println!("Height diff: {}", height_diff);
            let position = already_occured
                .iter()
                .position(|c_item| c_item == &current)
                .unwrap();
            let amount_of_occurences = (rounds) / (i - position as u64);
            points = amount_of_occurences
                * (game_2.total_max_height - height_diff - height_map[position].0);
            println!("Repeats at iteration: {}, Position: {}", i, position);

            let old_height = game_2.total_max_height;
            for _ in 0..((rounds - position as u64) % (i - position as u64)) {
                game_2.move_rock();
                game_2.prepare_next_rock();
            }
            points += game_2.total_max_height - old_height + height_map[position].0;

            break;
        }
        already_occured.push((
            game_2.current_instruction_position,
            game_2.current_rock.shape,
        ));
        height_map.push((
            game_2.total_max_height,
            game_2.total_max_height - last_height,
        ));
        game_2.prepare_next_rock();
    }

    //println!("{}", game_2);

    //part 2
    println!("Points 2:\t{}", points);

    Ok(())
}
