use std::env::args;

mod load;
use load::*;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    let len = data.first().unwrap().len();

    let common_pair = data
        .iter()
        .fold(
            [(0u64, 0u64)].iter().cycle().take(len).copied().collect::<Vec<_>>(),
            |acc, vec|
                acc
                    .iter()
                    .zip(vec)
                    .map(|(&(f, t), &b)| if b {
                        (f, t+1)
                    } else {
                        (f+1, t)
                    })
                    .collect()
        );
    let common_bool = common_pair
        .iter()
        .map(|(f, t)| t > f)
        .collect::<Vec<_>>();

    {
        let gamma = common_bool
            .iter()
            .fold(0u64, |a, &b| a*2 + if b { 1 } else { 0 });
        let epsilon = common_bool
            .iter()
            .fold(0u64, |a, &b| a*2 + if b { 0 } else { 1 });
        let power = gamma * epsilon;
        println!("Part 1: {}", power);
    }

    {
        let oxygen_bool = (0usize..len)
            .fold(data.clone(), |remaining, bit| {
                match remaining.len() {
                    1 => remaining,
                    _ => {
                        let common = {
                            let (t, f) = remaining
                                .iter()
                                .map(|x| x[bit])
                                .fold((0u64, 0u64), |(t, f), b| if b {
                                    (t+1, f)
                                } else {
                                    (t, f+1)
                                });
                            if t >= f { true } else { false }
                        };
                        remaining
                            .iter()
                            .filter(|x| x[bit] == common)
                            .cloned()
                            .collect()
                    }
                }
            });
        let oxygen = oxygen_bool
            .first()
            .unwrap()
            .iter()
            .fold(0u64, |a, &b| a*2 + if b { 1 } else { 0 });
        let co2_bool = (0usize..len)
            .fold(data.clone(), |remaining, bit| {
                match remaining.len() {
                    1 => remaining,
                    _ => {
                        let common = {
                            let (t, f) = remaining
                                .iter()
                                .map(|x| x[bit])
                                .fold((0u64, 0u64), |(t, f), b| if b {
                                    (t+1, f)
                                } else {
                                    (t, f+1)
                                });
                            if t >= f { true } else { false }
                        };
                        remaining
                            .iter()
                            .filter(|x| x[bit] != common)
                            .cloned()
                            .collect()
                    }
                }
            });
        let co2 = co2_bool
            .first()
            .unwrap()
            .iter()
            .fold(0u64, |a, &b| a*2 + if b { 1 } else { 0 });
        let life = oxygen * co2;
        println!("Part 2: {}", life);
    }

    Ok(())
}
