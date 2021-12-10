use std::cell::RefCell;
use std::cmp::{min, PartialEq};
use crate::field::GameStatus::{COMPUTER_WINS, DRAW, GOING, PLAYER_WINS};
use crate::gameplay::Gameplay;


#[derive(PartialEq)]
pub enum GameStatus {
    GOING = 0,
    PLAYER_WINS = 1,
    COMPUTER_WINS = 2,
    DRAW = 3
}

pub static PLAYER:   char = 'X';
pub static COMPUTER: char = 'O';
pub static EMPTY:    char = '~';


#[derive(Debug)]
pub struct Move {
    pub row: i32,
    pub col: i32,
}


#[derive(Debug, Clone)]
pub struct Field {
    pub field: [[char; 3]; 3],
}

impl Field {
    pub fn default() -> Self {
        Self {
            field: [[EMPTY; 3]; 3],
        }
    }

    pub fn draw(&self) {
        println!("- - - - - -");
        for i in self.field {
            for j in i {
                print!("|{}| ", j);
            }
            println!("\n- - - - - -");
        }
    }

    pub fn is_win(&self) -> GameStatus {
        let result = GOING;


        // checking for ME winning
        for i in 0..3 as usize {
            let mut counter = 0;
            for j in 0..3 as usize {
                if self.field[j][i] == PLAYER {
                    counter += 1;
                }
            }
            if counter == 3 {
                return PLAYER_WINS;
            }
        }


        // check if win in row
        for i in 0..3 as usize {
            let mut counter = 0;
            for j in 0..3 as usize {
                if self.field[i][j] == PLAYER {
                    counter += 1;
                }
            }
            if counter == 3 {
                return PLAYER_WINS;
            }


            // check if PLAYER win in diagonal
            if (self.field[0][0] == PLAYER && self.field[1][1] == PLAYER && self.field[2][2] == PLAYER) ||
                (self.field[0][2] == PLAYER && self.field[1][1] == PLAYER && self.field[2][0] == PLAYER) {
                return PLAYER_WINS;
            }
        }

        // cheking for COMPUTER win
        for i in 0..3 as usize {
            let mut counter = 0;
            for j in 0..3 as usize {
                if self.field[j][i] == COMPUTER {
                    counter += 1;
                }
            }
            if counter == 3 {
                return COMPUTER_WINS;
            }
        }

        for i in 0..3 as usize {
            let mut counter = 0;
            for j in 0..3 as usize {
                if self.field[i][j] == COMPUTER {
                    counter += 1;
                }
            }
            if counter == 3 {
                return COMPUTER_WINS;
            }
        }


        // check if COMPUTER win in diagonal
        if (self.field[0][0] == COMPUTER && self.field[1][1] == COMPUTER && self.field[2][2] == COMPUTER) ||
            (self.field[0][2] == COMPUTER && self.field[1][1] == COMPUTER && self.field[2][0] == COMPUTER) {
            return COMPUTER_WINS;
        }

        let mut is_full = true;
        for i in 0..3 as usize {
            for j in 0..3 as usize {
                if self.field[i][j] == EMPTY {
                    is_full = false;
                    break;
                }
            }
        }
        if is_full {
            return DRAW;
        }
        result
    }
}


pub fn make_best_move(field: &mut Field) -> Move{
    let mut best_val = -1000;
    let mut best_move = Move{row: -1, col: -1};
    for i in 0..3 as usize {
        for j in 0..3 as usize {
            if field.field[i][j] == EMPTY {
                field.field[i][j] = COMPUTER;

                let move_val = minimax(field, 1, true);

                field.field[i][j] = EMPTY;

                if move_val > best_val {
                    best_move.row = i as i32;
                    best_move.col = j as i32;
                    best_val = move_val;
                }
            }
        }
    }
    best_move
}


fn minimax(field: &mut Field, depth: i32, is_max: bool) -> i32{
    let score = score(field);

    match score {
        10 => return 10,
        -10 => return -10,
        0 => return 0,
        _ => (),
    }

    if is_max {
        let mut best = -1000;
        for i in 0..3 as usize {
            for j in 0..3 as usize {
                if field.field[i][j] == EMPTY {
                    let mut tmp_field = field.clone();
                    tmp_field.field[i][j] = COMPUTER;
                    best = *[best, minimax(&mut tmp_field, depth+1, !is_max)].iter().max().unwrap();
                }
            }
        }
        return best;
    }
    else {
        let mut best = 1000;
        for i in 0..3 as usize {
            for j in 0..3 as usize {
                if field.field[i][j] == EMPTY {
                    let mut tmp_field = field.clone();
                    tmp_field.field[i][j] = PLAYER;
                    best = *[best, minimax(&mut tmp_field, depth+1, !is_max)].iter().min().unwrap();
                }
            }
        }
        return best;
    }
    0
}



fn score(field: &Field) -> i32 {
    let game_status = field.is_win();
    return if game_status == COMPUTER_WINS {
        10
    } else if game_status == PLAYER_WINS {
        -10
    } else{
        0
    }
}

