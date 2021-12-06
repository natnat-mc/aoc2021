use std::env::args;

mod load;
use load::*;

mod data;
use data::*;

pub type Error = &'static str;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data: {:?}", data);

    {
        let count = State::from(&data).steps(80).total();
        println!("Part 1: {}", count);
    }

    {
        let count = State::from(&data).steps(256).total();
        println!("Part 1: {}", count);
    }

    Ok(())
}
