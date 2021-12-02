pub type Error = &'static str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Forward
}
pub use Direction::{Up, Down, Forward};

impl Direction {
    pub fn from(txt: &str) -> Result<Direction, Error> {
        match txt {
            "forward" => Ok(Forward),
            "up" => Ok(Up),
            "down" => Ok(Down),
            _ => Err("Unrecognized direction")
        }
    }
}

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<Vec<(Direction, i64)>, Error> {
    input
        .as_ref()
        .split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();
            let (left, right) = (split[0], split[1]);
            Ok((Direction::from(left)?, right.parse::<i64>().map_err(|_| "Failed to parse number")?))
        })
        .collect::<Result<Vec<_>, _>>()
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<Vec<(Direction, i64)>, Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to read file")?
    )
}