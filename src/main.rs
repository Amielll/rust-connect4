mod game;
use crossterm::{terminal, Result};
use game::controller;
use game::model::*;
use game::view;

fn main() -> Result<()> {
    let mut board = Board::new(Colour::Red);
    view::print_grid(&board.grid);

    while board.running {
        terminal::enable_raw_mode()?;
        view::print_prompt(&board)?;
        if !controller::process_input(&mut board)? {
            continue;
        };
        terminal::disable_raw_mode()?;

        board.play_turn();
        view::print_grid(&board.grid);
        if let Some(c) = board.check_win() {
            view::print_gameover(c);
        }
        board.cursor = 3;
    }

    Ok(())
}
