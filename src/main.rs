use std::fs::File;
use std::io::prelude::*;
use std::io;
use ansi_term;

fn main() {
    let mut board = Board::new(Colour::Red);
    board.print_grid();
    while board.running {
        let mut col: u8; 
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            col = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if board.is_valid_move(col.into()) { break; }
            else { continue }
        }
        
        board.play_turn(col.into());
        board.print_grid();

        let win_state: Option<Colour> = board.check_win();
        if win_state != None {
            match win_state.unwrap() {
                Colour::Red => println!("Red wins!"),
                Colour::Yellow => println!("Yellow wins!"),
            }
            board.running = false;
        }

    }  
}

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Red,
    Yellow,
}

#[derive(Copy, Clone)]
pub struct Board {
    pub grid: [Option<Colour>; 42], // 7x6 grid
    pub turn: Colour,
    pub running: bool
}

impl Board {
    pub fn new(start_colour: Colour) -> Self {
        Board {
            grid: [None; 42], 
            turn: start_colour,
            running: true
        }
    }

    pub fn is_valid_move(&self, col: usize) -> bool {
        // Valid move if specified column in top row is empty
        if col >= 7 { return false }
        match self.grid[col] { Some(_) => false, None => true }
    }

    pub fn print_grid(&self) { 
        let mut row: u8 = 0;
        print!("{esc}c", esc = 27 as char);
        println!("┌───────┐"); 

        while row < 6 {
            for col in 0..7 {
                let ind: usize = (col + row * 7).into();

                if ind % 7 == 0 { print!("│") } // Left border

                match self.grid[ind] {
                    Some(c) => {
                        match c {
                            Colour::Red => {
                                print!("{}", ansi_term::Colour::Red.paint("O"));
                            },
                            Colour::Yellow => {
                                print!("{}", ansi_term::Colour::Yellow.paint("O"));
                            },
                        }
                    },
                    None => { print!(" "); }
                }
            }

            println!("│"); // Right border
            row += 1;
        }

        println!("└───────┘");
    }

    pub fn play_turn(&mut self, col: usize) {
        // "Drops" the current player's piece into the grid 
        let mut ind: usize = col;
        loop {
            match self.grid[ind + 7] {
                Some(_) => break,
                None => ind += 7,
            }

            if ind + 7 > 41 { break; }
        }

        self.grid[ind] = Some(self.turn);
       
        match self.turn {
            Colour::Red => self.turn = Colour::Yellow,
            Colour::Yellow => self.turn = Colour::Red,
        }

    }

    pub fn check_win(&self) -> Option<Colour> {
        // Checks entire board for 4 in a row
        
        let mut w = false;
        for i in 0..42 {
            if self.grid[i] == None { continue; }
            let c: Colour = self.grid[i].unwrap();

            // Horizontal check (left to right)
            if i % 7 < 4 { // Ignore 3 rightmost columns
                for delta in 1..4 {
                    if self.grid[i+delta] != None { // Avoid unwrapping None
                        if self.grid[i+delta].unwrap() != c { break; }
                        if delta == 3 { w = true; }
                    } else { break; } // Adjacent piece is None, no 4 in a row
                }

                if w { return Some(c); }
            }


            // Vertical check (this check and the next two are descending)
            if i < 20 { // Ignore bottom 3 rows
                for delta in 1..4 {
                    if self.grid[i+7*delta] != None {
                        if self.grid[i+7*delta].unwrap() != c { break; }
                        if delta == 3 { w = true; }
                    } else { break; } 
                }

                if w { return Some(c); }
            }


            // Right diagonal check
            if i < 17 { // Last possible index for diag to start
                for delta in 1..4 {
                    if self.grid[i+8*delta] != None {
                        if (i+8*delta) % 7 == 0 { break; } // Avoid wraparound
                        if self.grid[i+8*delta].unwrap() != c { break; }
                        if delta == 3 { w = true; }
                    } else { break; }
                }
                if w { return Some(c); }
            }


            // Left diagonal check
            if i < 20 { // Last possible index for diag to start
                for delta in 1..4 {
                    if self.grid[i+6*delta] != None {
                        if (i+6*delta) % 7 == 6 { break; } // Avoid wraparound
                        if self.grid[i+6*delta].unwrap() != c { break; }
                        if delta == 3 { w = true; }
                    } else { break; }
                }
            }
            if w { return Some(c); }
        }

        None
    }

}



