mod game;
use game::model::*;
use game::view;
use game::controller;
use crossterm::{Result, terminal};

fn main() -> Result<()> {
    let mut board = Board::new(Colour::Red);
    view::print_grid(board.grid);

    while board.running {
        terminal::enable_raw_mode()?; 
        view::print_prompt(board)?;
        if !controller::process_input(&mut board)? {
            continue;
        };
        terminal::disable_raw_mode()?;  

        board.play_turn(board.cursor);
        view::print_grid(board.grid);
        board.check_win();
        board.cursor = 3;

    }  

    Ok(())
}
