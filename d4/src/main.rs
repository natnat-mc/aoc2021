use std::env::args;

mod load;
use load::*;
mod board;
use board::*;

pub type Error = &'static str;

fn main() -> Result<(), Error> {
    let data = load_data_from_file(args().skip(1).next().expect("Requires input file as argument"))?;
    println!("Loaded data");

    enum State {
        Finished(usize, Board),
        Running(Vec<Board>)
    }
    impl State {
        fn finished_or(self) -> Result<(usize, Board), Error> {
            match self {
                State::Finished(num, board) => Ok((num, board)),
                State::Running(_) => Err("not finished")
            }
        }
    }

    {
        let winner = data
            .0
            .iter()
            .fold(State::Running(data.1.clone()), |state, &number| match state {
                State::Finished(id, board) => State::Finished(id, board),
                State::Running(boards) => {
                    let boards = boards
                        .iter()
                        .map(|board| board.set(number))
                        .collect::<Vec<_>>();
                    match boards
                        .iter()
                        .enumerate()
                        .find_map(|(id, board)| if board.is_won() {
                            Some((id, board))
                        } else {
                            None
                        })
                    {
                        Some((id, board)) => State::Finished(id, board.clone()),
                        None => State::Running(boards)
                    }
                }
            })
            .finished_or()
            ?;
        println!("Part 1: {}", winner.1.get_score());
    }

    {
        let loser = data
            .0
            .iter()
            .fold(data.1.clone(), |boards, &number| boards
                .iter()
                .map(|board| board.set(number))
                .collect::<Vec<_>>()
            )
            .iter()
            .fold(Option::<Board>::None, |board, current| match board {
                None => Some(current.clone()),
                Some(board) => if board.get_rounds() < current.get_rounds() {
                    Some(current.clone())
                } else {
                    Some(board)
                }
            })
            .ok_or_else(|| "no board")
            ?;
        println!("Part 2: {}", loser.get_score());
    }

    Ok(())
}
