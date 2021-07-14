use std::io;
use ansi_term;

fn main() {
    let mut board = Board::new(Colour::Red);

    while board.running {
        let mut col: u8;

        board.print_grid();

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
        
    }  
}

#[derive(Copy, Clone)]
pub enum Colour {
    Red,
    Yellow,
}

#[derive(Copy, Clone)]
pub struct Board {
    pub grid: [Option<Colour>; 42],
    pub turn: Colour,
    pub running: bool
}

impl Board {
    pub fn new(start_colour: Colour) -> Self {
        Board {
            grid: [None; 42], // 7x6 grid
            turn: start_colour,
            running: true
        }
    }

    pub fn is_valid_move(&self, col: usize) -> bool {
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
                    Some(color) => {
                        match color {
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

}



