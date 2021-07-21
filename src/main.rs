use std::io;

mod game;
use game::*;
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
        board.check_win();

    }  
}
