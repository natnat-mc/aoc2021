use crate::Error;

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<Vec<u32>, Error> {
    input
        .as_ref()
        .trim()
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Failed to parse number")
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<Vec<u32>, Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to read file")?
    )
}
