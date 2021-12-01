pub type Error = &'static str;

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<Vec<u64>, Error> {
    input
        .as_ref()
        .split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|_| "Failed to parse data")
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<Vec<u64>, Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to rerad file")?
    )
}