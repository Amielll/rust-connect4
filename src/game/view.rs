use super::model::Colour;
use super::model::Board;
use std::io::{Write, stdout, Error};
use std::result::Result;
use crossterm::{execute, cursor, terminal};

pub fn print_grid(grid: [Option<Colour>; 42]) {
    print!("{esc}c", esc = 27 as char);
    println!("┌───────┐");

    for row in 0..6 {
        for col in 0..7 {
            let ind: usize = col + row * 7; 
            if ind % 7 == 0 { print!("│") } // Left border

            match grid[ind] {
                Some(c) => {
                    match c {
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
    }
    println!("└───────┘");
}


pub fn print_prompt(board: Board) -> Result<(), Error> {
    let mut stdout = stdout();  
 
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)); 
    for _ in 0..board.cursor {
        write!(stdout, " ")?;
    }
    write!(stdout, " {}\r\n", ansi_term::Colour::Green.paint("^"))?;
    match board.turn {
        Colour::Red => write!(stdout, "Turn: {}\r\n", ansi_term::Colour::Red.paint("red"))?,
        Colour::Yellow => write!(stdout, "Turn: {}\r\n", ansi_term::Colour::Yellow.paint("yellow"))?,
    };
    execute!(stdout, cursor::MoveUp(2));
    Ok(())
}
