use std::env::args;

pub type Error = &'static str;

mod load;
use load::*;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    let max = data
        .iter()
        .fold(0, |a, &b| if a > b {
            a
        } else {
            b
        });

    {
        let fuel = (0..=max)
            .map(|align| data
                .iter()
                .map(|&horz| if horz > align {
                    horz - align
                } else {
                    align - horz
                })
                .sum::<u32>()
            )
            .min()
            .unwrap();
        println!("Part 1: {}", fuel);
    }

    {
        let fuel = (0..=max)
            .map(|align| data
                .iter()
                .map(|&horz| if horz > align {
                    horz - align
                } else {
                    align - horz
                })
                .map(|dist| dist*(dist+1)/2)
                .sum::<u32>()
            )
            .min()
            .unwrap();
        println!("Part 2: {}", fuel);
    }

    Ok(())
}
