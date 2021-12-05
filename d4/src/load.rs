use crate::{Board, Error};

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<(Vec<u64>, Vec<Board>), Error> {
    let text = input
        .as_ref()
        .trim()
        .split("\n\n")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    let numbers = text
        .iter()
        .next()
        .ok_or_else(|| "failed to get numbers")?
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<_, _>>()
        .map_err(|_| "failed to parse numbers")
        ?;
    let boards = text
        .iter()
        .skip(1)
        .map(Board::from_string)
        .collect::<Result<_, _>>()
        ?;
    Ok((numbers, boards))
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<(Vec<u64>, Vec<Board>), Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to read file")?
    )
}
