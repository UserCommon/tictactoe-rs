use std::io::stdin;
use crate::field::*;
use crate::field::GameStatus::*;

pub struct Gameplay {
    pub turn: bool, // my turn == 0
    pub end: bool
}

impl Gameplay {
    pub fn new() -> Self {
        Gameplay {
            turn: true,
            end:  false,
        }
    }

    pub fn event_loop(&mut self, field: &mut Field) {
        loop {
            println!("Say row and column");

            let mut pos = String::new();
            stdin().read_line(&mut pos).unwrap();

            let step: Vec<usize> = pos.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
            if step.len() == 2 && field.field[step[0]-1][step[1]-1] == EMPTY {
                field.field[step[0]-1][step[1]-1] = PLAYER;
            }
            else {
                println!("Wrong input, try again!");
                continue;
            }
            field.draw();
            self.turn = false;
            if self.is_end(field) {
                break
            };

            self.computer_step(field);
            field.draw();
            self.turn = true;
            if self.is_end(field) {
                break;
            };

        }
        println!("Game Over!");
    }

    fn computer_step(&mut self, field: &mut Field) {
        println!("{:?}", field);
        let res = make_best_move(field);
        println!("{:?}", res);
        field.field[res.row as usize][res.col as usize] = COMPUTER;
    }


    pub fn is_end(&mut self, field: &Field) -> bool {
        if field.is_win() != GOING {
            self.end = true
        }
        return if self.end { true } else { false }

    }

}