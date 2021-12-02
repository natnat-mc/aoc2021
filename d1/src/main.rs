use std::env::args;

mod load;
use load::*;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    {
        let mut last = u64::MAX;
        let mut increases = 0u64;
        for &val in data.iter() {
            if val > last {
                increases += 1;
            }
            last = val;
        }
        println!("Part 1: {}", increases);
    }

    {
        let (_, increases) = data
            .iter()
            .fold((u64::MAX, 0u64), |(last, increases), &val| if val > last {
                (val, increases+1)
            } else {
                (val, increases)
            });
        println!("Part 1 (fold): {}", increases);
    }

    {
        let mut last = u64::MAX;
        let mut increases = 0u64;
        for val in data.windows(3).map(|x| x.iter().fold(0, |a, &b| a+b)) {
            if val > last {
                increases += 1;
            }
            last = val;
        }
        println!("Part 2: {}", increases);
    }

    {
        let (_, increases) = data
            .windows(3)
            .map(|x| x.iter().fold(0, |a, &b| a+b))
            .fold((u64::MAX, 0u64), |(last, increases), val| if val > last {
                (val, increases+1)
            } else {
                (val, increases)
            });
        println!("Part 2 (fold): {}", increases);
    }

    Ok(())
}
