use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use game::Game;

mod game;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let mut games: Vec<Game> = vec![];
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let game: Game = Game::new(line?);
        games.push(game);
    }

    let games_score: i32 = games.iter().map(|c_game| c_game.get_points_basic()).sum();

    println!("Score basic:\t{}", games_score);

    let games_score_advanced: i32 = games
        .iter()
        .map(|c_game| c_game.get_points_advanced())
        .sum();

    println!("Score advanced:\t{}", games_score_advanced);

    Ok(())
}
