use crate::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    data: Vec<Vec<(u64, bool)>>,
    last: u64,
    rounds: u64
}

impl Board {
    pub fn from_string<S: AsRef<str>>(string: S) -> Result<Board, Error> {
        Ok(Board {
            data: string
                .as_ref()
                .split('\n')
                .map(|line| line
                    .split_ascii_whitespace()
                    .map(|x| Ok((
                        x.parse().map_err(|_| "failed to parse number")?,
                        false
                    )))
                    .collect::<Result<Vec<_>, _>>()
                )
                .collect::<Result<Vec<_>, _>>()
                ?,
            last: 0,
            rounds: 0
        })
    }

    pub fn set(&self, number: u64) -> Board {
        if self.is_won() {
            return self.clone()
        }
        Board {
            data: self
                .data
                .iter()
                .map(|x| x
                    .iter()
                    .map(|&y| (
                        y.0,
                        if number == y.0 {
                            true
                        } else {
                            y.1
                        }
                    ))
                    .collect()
                )
                .collect(),
            last: number,
            rounds: self.rounds + 1
        }
    }

    pub fn is_won(&self) -> bool {
        self
            .data
            .iter()
            .any(|x| x
                .iter()
                .all(|&y| y.1)
            )
            || (0..5)
            .any(|n| self
                .data
                .iter()
                .flatten()
                .skip(n)
                .step_by(5)
                .all(|&x| x.1)
            )
    }

    pub fn get_score(&self) -> u64 {
        self
            .data
            .iter()
            .flatten()
            .filter_map(|&(a, b)| if !b {
                Some(a)
            } else {
                None
            })
            .sum::<u64>()
            * self.last
    }

    pub fn get_rounds(&self) -> u64 {
        self.rounds
    }
}
