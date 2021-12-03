pub type Error = &'static str;

pub fn load_data_from_string<S: AsRef<str>>(input: S) -> Result<Vec<Vec<bool>>, Error> {
    input
        .as_ref()
        .split('\n')
        .filter(|line| line.len() != 0)
        .map(|line|
            line
                .as_bytes()
                .iter()
                .map(|&b| match b {
                    0x30 => Ok(false),
                    0x31 => Ok(true),
                    _ => Err("Invalid binary digit")
                })
                .collect::<Result<Vec<_>, _>>()
        )
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Failed to parse data")
}
pub fn load_data_from_file<S: AsRef<str>>(path: S) -> Result<Vec<Vec<bool>>, Error> {
    load_data_from_string(
        std::fs::read_to_string(path.as_ref()).map_err(|_| "Failed to read file")?
    )
}