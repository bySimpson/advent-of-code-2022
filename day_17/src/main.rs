use std::{fs, io};

use clap::{command, Parser};

use crate::items::{Game, Move};

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
    }

    println!("Points 1:\t{}", game.total_max_height);

    for i in 0..(10091u64) {
        game_2.move_rock();
        if i % 1000000 == 0 {
            println!("{}", i);
        }
    }

    //part 2
    println!("Points 2:\t{}", game_2.total_max_height);

    Ok(())
}
