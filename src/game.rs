use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

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

    fn is_valid_move(&self, col: usize) -> bool {
        // Valid move if specified column in top row is empty
        if col >= 7 { return false }
        self.grid[col].is_none()
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

    pub fn process_input(&self) -> crossterm::Result<u8> {
        //TODO: Need to print cursor with respect to location
        let mut cursor: u8 = 3; // Start the cursor in the center

        loop {  
            match read()? {
                Event::Key(key_event) => {

                    match key_event.code {
                        KeyCode::Left | KeyCode::Char('a') => {
                            if cursor > 0 { cursor -= 1;}
                        },
                        KeyCode::Right | KeyCode::Char('d') => {
                            if cursor < 6 { cursor += 1;}
                        }
                        KeyCode::Enter => {
                            if self.is_valid_move(cursor.into()) { break; }
                        },
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }

        Ok(cursor)

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

    pub fn check_win(&mut self) {
        // Checks entire board for 4 in a row 
        for i in 0..42 {
            if self.grid[i] == None { continue; }
            let c: Colour = self.grid[i].unwrap();

            // Horizontal check (left to right)
            if i % 7 < 4 { // Ignore 3 rightmost columns
               if self._check_win_helper(i, c, 1, 7) {
                   self.finish_game(c);
                }
            }

            // Vertical check (this check and the next two are descending)
            if i < 20 { // Ignore bottom 3 rows
                if self._check_win_helper(i, c, 7, 7) {
                    self.finish_game(c);
                }
            }

            // Right diagonal check
            if i < 18 { // Last possible index for diag to start
                if self._check_win_helper(i, c, 8, 0) {
                    self.finish_game(c);
                } 
            }

            // Left diagonal check
            if i < 21 { // Last possible index for diag to start
                if self._check_win_helper(i, c, 6, 6) { 
                    self.finish_game(c); 
                }
            } 
        }
    }

    fn _check_win_helper(&self, ind: usize, target: Colour, mult: usize, wraparound: usize) -> bool {
        /* Checks the next 3 pieces to see if 4 in a row was formed
         * ind - index to start delta shift at
         * target - colour piece to match 4 in a row in
         * mult - delta multiplier, used to traverse grid in different directions
         * wraparound - index to detect grid wrap (7 passed to ignore this check)
         */
        let mut win = false;
        for delta in 1..4 {
            if self.grid[ind + mult*delta] != None {
                if (ind + mult*delta) % 7 == wraparound { break; } // Avoid grid wrap
                if self.grid[ind + mult*delta].unwrap() != target { break;}
                if delta == 3 { win = true; } // If we made it here, found 4 in a row
            } else { break;}
        }
        win
    }

    fn finish_game(&mut self, colour: Colour) {
        match colour {
            Colour::Red => println!("{} wins!", ansi_term::Colour::Red.paint("Red")),
            Colour::Yellow => println!("{} wins!", ansi_term::Colour::Yellow.paint("Yellow")),
        }
        self.running = false;
    }

}
