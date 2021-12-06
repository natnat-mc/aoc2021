#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct State {
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
}

impl State {
    fn new() -> State {
        State {
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            a8: 0,
        }
    }
    pub fn from(data: &Vec<u8>) -> State {
        data
            .iter()
            .fold(Self::new(), |state, &num| state.add(num))
    }

    fn add(self, num: u8) -> State {
        match num {
            0 => State { a0: self.a0 + 1, ..self },
            1 => State { a1: self.a1 + 1, ..self },
            2 => State { a2: self.a2 + 1, ..self },
            3 => State { a3: self.a3 + 1, ..self },
            4 => State { a4: self.a4 + 1, ..self },
            5 => State { a5: self.a5 + 1, ..self },
            6 => State { a6: self.a6 + 1, ..self },
            7 => State { a7: self.a7 + 1, ..self },
            8 => State { a8: self.a8 + 1, ..self },
            _ => panic!("invalid number")
        }
    }

    pub fn step(self) -> State {
        State {
            a0: self.a1,
            a1: self.a2,
            a2: self.a3,
            a3: self.a4,
            a4: self.a5,
            a5: self.a6,
            a6: self.a0 + self.a7,
            a7: self.a8,
            a8: self.a0
        }
    }

    pub fn steps(self, steps: u64) -> State {
        (0..steps)
            .fold(self, |state, _|
                state.step()
            )
    }

    pub fn total(self) -> u64 {
        self.a0
            + self.a1
            + self.a2
            + self.a3
            + self.a4
            + self.a5
            + self.a6
            + self.a7
            + self.a8
    }
}
