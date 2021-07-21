mod game;
use game::model::*;
use game::view;
use game::controller;

fn main() {
    let mut board = Board::new(Colour::Red);
    view::print_grid(board.grid);

    while board.running {
        let col: u8;

        match controller::process_input(board) {
            Ok(num) => col = num,
            _ => panic!(),
        }
         
        board.play_turn(col.into());
        view::print_grid(board.grid);
        board.check_win();

    }  
}
