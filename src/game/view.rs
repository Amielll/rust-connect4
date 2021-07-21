use super::model::Colour;

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
