use std::env::args;

mod load;
use load::*;
mod data;
use data::*;

pub type Error = &'static str;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    {
        let count = data
            .iter()
            .fold(Board::new(), |board, &line| if line.is_cardinal() {
                board.add_line(line)
            } else {
                board
            })
            .overlaps();
        println!("Part 1: {}", count);
    }

    /*{
        data
            .iter()
            .fold(Board::new(), |board, &line|
                board.add_line(line)
            )
            .draw();
    }*/

    {
        let count = data
            .iter()
            .fold(Board::new(), |board, &line|
                board.add_line(line)
            )
            .overlaps();
        println!("Part 2: {}", count);
    }

    Ok(())
}
