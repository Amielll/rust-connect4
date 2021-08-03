#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Red,
    Yellow,
}


#[derive(Copy, Clone)]
pub struct Board {
    pub grid: [Option<Colour>; 42], // 7x6 grid
    pub turn: Colour,
    pub running: bool,
    pub cursor: usize,
}


impl Board {
    pub fn new(start_colour: Colour) -> Self {
        Board {
            grid: [None; 42],
            turn: start_colour,
            running: true,
            cursor: 3,  // Place cursor in middle of board
        }
    }

    pub fn is_valid_move(&self) -> bool {
        // Valid move if specified column in top row is empty 
        self.grid[self.cursor].is_none()
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

    // TODO: Move this to view layer
    fn finish_game(&mut self, colour: Colour) {
        match colour {
            Colour::Red => println!("{} wins!", ansi_term::Colour::Red.paint("Red")),
            Colour::Yellow => println!("{} wins!", ansi_term::Colour::Yellow.paint("Yellow")),
        }
        self.running = false;
    }

}
