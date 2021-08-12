use super::model::Board;
use crossterm::event::{read, Event, KeyCode};
use std::process;

pub fn process_input(board: &mut Board) -> crossterm::Result<bool> {
    let mut move_status: bool = false; // Indicates if piece should drop
    if let Event::Key(key_event) = read()? {
        match key_event.code {
            KeyCode::Left | KeyCode::Char('a') => {
                if board.cursor > 0 {
                    board.cursor -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('d') => {
                if board.cursor < 6 {
                    board.cursor += 1;
                }
            }
            KeyCode::Enter => {
                move_status = board.is_valid_move();
            }
            KeyCode::Char('q') | KeyCode::Esc => process::exit(0),
            _ => (),
        }
    }

    Ok(move_status)
}
