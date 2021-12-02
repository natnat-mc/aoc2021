use std::env::args;

mod load;
use load::*;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    {
        let (horz, vert) = data
            .iter()
            .fold((0i64, 0i64), |(horz, vert), &(action, dist)| match action {
                Forward => (horz+dist, vert),
                Up => (horz, vert-dist),
                Down => (horz, vert+dist)
            });
        let val = horz * vert;
        println!("Part 1: {}", val);
    }

    {
        let (horz, vert, _) = data
            .iter()
            .fold((0i64, 0i64, 0i64), |(horz, vert, aim), &(action, dist)| match action {
                Forward => (horz+dist, vert+aim*dist, aim),
                Up => (horz, vert, aim-dist),
                Down => (horz, vert, aim+dist)
            });
        let val = horz * vert;
        println!("Part 2: {}", val);
    }

    Ok(())
}
