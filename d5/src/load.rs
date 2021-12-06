use crate::{Line, Error};

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<Vec<Line>, Error> {
    input
        .as_ref()
        .trim()
        .split("\n")
        .map(Line::parse)
        .collect()
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<Vec<Line>, Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to read file")?
    )
}
