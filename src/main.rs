mod game;
use game::*;

fn main() {
    let mut board = Board::new(Colour::Red);
    board.print_grid();

    while board.running {
        let col: u8;

        match board.process_input() {
            Ok(num) => col = num,
            _ => panic!(),
        }
         
        board.play_turn(col.into());
        board.print_grid();
        board.check_win();

    }  
}
