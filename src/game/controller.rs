use crossterm::event::{read, Event, KeyCode};
use super::model::Board;

pub fn process_input(board: Board) -> crossterm::Result<u8> {
    //TODO: Need to print cursor with respect to location
    let mut cursor: u8 = 3; // Start the cursor in the center

    loop {
        for _ in 0..cursor {
            print!(" ");
        }
        println!(" {}", ansi_term::Colour::Green.paint("^"));

        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Left | KeyCode::Char('a') => if cursor > 0 { cursor -= 1; },
                KeyCode::Right | KeyCode::Char('d') => if cursor < 6 { cursor += 1; },
                KeyCode::Enter => {
                    if board.is_valid_move(cursor.into()) { break; }
                },
                _ => (),
            }
        }

    }

    Ok(cursor)

}
