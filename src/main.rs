use std::io;

fn main() {
    let mut board = Board::new(Colour::Red);

    while board.running {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

       let col: u8 = match input.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
        };
       
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
            grid: [None; 42],
            turn: start_colour,
            running: true
        }
    }

    pub fn is_valid_move(&self, col: usize) -> bool {
        match self.grid[col] { Some(_) => false, None => true }
    }

    pub fn print_board() { 
        print!("{esc}c", esc = 27 as char);
        println!("┌───────┐");

        for _ in 0..6 {
            println!("│1234567│");
        }

        println!("└───────┘");
    }
}



