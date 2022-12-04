#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    RockLoose = 1,
    PaperDraw = 2,
    ScissorsWin = 3,
}

pub struct Game {
    choice_player_result: Choice,
    choice_opponent: Choice,
}

impl Game {
    pub fn new(game_line: String) -> Self {
        let split_command_line: Vec<&str> = game_line.split(" ").collect();
        let choice_opponent: Choice = match split_command_line[0] {
            "A" => Choice::RockLoose,
            "B" => Choice::PaperDraw,
            "C" => Choice::ScissorsWin,
            _ => panic!("Unknown command!"),
        };
        let choice_player: Choice = match split_command_line[1] {
            "X" => Choice::RockLoose,
            "Y" => Choice::PaperDraw,
            "Z" => Choice::ScissorsWin,
            _ => panic!("Unknown command!"),
        };
        Self {
            choice_player_result: choice_player,
            choice_opponent,
        }
    }

    pub fn get_points_basic(&self) -> i32 {
        let mut points = if &self.choice_player_result == &self.choice_opponent {
            // draw
            3
        } else if self.choice_player_result as i32 - self.choice_opponent as i32 == 1
            || self.choice_player_result as i32 - self.choice_opponent as i32 == -2
        {
            // win
            6
        } else {
            // loose
            0
        };
        points += self.choice_player_result as i32;
        points
    }

    pub fn get_points_advanced(&self) -> i32 {
        let points = match self.choice_player_result {
            Choice::ScissorsWin => {
                let mut c_points = self.choice_opponent as i32 + 1;
                if c_points > 3 {
                    c_points = 1;
                }
                c_points
            }
            Choice::PaperDraw => {
                let c_points = self.choice_opponent as i32;
                c_points
            }
            Choice::RockLoose => {
                let mut c_points = self.choice_opponent as i32 - 1;
                if c_points < 1 {
                    c_points = 3;
                }
                c_points
            }
        };
        points
            + match self.choice_player_result {
                Choice::RockLoose => 0,
                Choice::PaperDraw => 3,
                Choice::ScissorsWin => 6,
            }
    }
}
