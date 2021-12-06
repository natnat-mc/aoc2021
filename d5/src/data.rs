use crate::Error;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Line {
    x0: u64, y0: u64,
    x1: u64, y1: u64,
}

impl Line {
    fn new(x0: u64, y0: u64, x1: u64, y1: u64) -> Line {
        Line {
            x0, y0,
            x1, y1,
        }
    }

    pub fn new_cell(a: Cell, b: Cell) -> Line {
        let (a, b) = if a.x < b.x {
            (a, b)
        } else {
            (b, a)
        };
        Self::new(a.x, a.y, b.x, b.y)
    }

    pub fn parse(data: &str) -> Result<Line, Error> {
        let points = data
            .split(" -> ")
            .map(Cell::parse)
            .collect::<Result<Vec<_>, _>>()
            ?;
        Ok(Self::new_cell(points[0], points[1]))
    }

    pub fn is_cardinal(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }

    pub fn get_coords(&self) -> Vec<Cell> {
        if self.x0 == self.x1 {
            let (y0, y1) = if self.y0 < self.y1 {
                (self.y0, self.y1)
            } else {
                (self.y1, self.y0)
            };
            (y0..=y1)
                .map(|y| Cell {x: self.x0, y})
                .collect()
        } else if self.y0 == self.y1 {
            (self.x0..=self.x1)
                .map(|x| Cell {x, y: self.y0})
                .collect()
        } else {
            let delta = self.y1 as i64 - self.y0 as i64;
            let (delta, sgn) = if delta < 0 { (-delta, -1) } else { (delta, 1) };
            (0..=delta)
                .map(|delta| Cell {
                    x: (self.x0 as i64 + delta) as u64,
                    y: (self.y0 as i64 + delta * sgn) as u64
                })
                .collect()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Cell {
    x: u64,
    y: u64
}

impl Cell {
    pub fn parse(data: &str) -> Result<Cell, Error> {
        let coords = data
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|_| "Failed to parse number")
            ?;
        Ok(Cell {
            x: coords[0],
            y: coords[1]
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    cells: Vec<(Cell, u64)>
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Vec::new()
        }
    }

    pub fn add(&self, cell: Cell) -> Board {
        let (x, y) = (cell.x, cell.y);
        let cells = {
            if self.cells.iter().any(|&(cell, _)| cell.x == x && cell.y == y) {
                self
                    .cells
                    .iter()
                    .map(|&(cell, count)| if cell.x == x && cell.y == y {
                        (cell, count+1)
                    } else {
                        (cell, count)
                    })
                    .collect()
            } else {
                self
                    .cells
                    .iter()
                    .chain([(Cell { x, y }, 1)].iter())
                    .cloned()
                    .collect()
            }
        };
        Board { cells }
    }

    pub fn add_line(&self, line: Line) -> Board {
        line
            .get_coords()
            .iter()
            .fold(self.clone(), |board, &cell|
                board.add(cell)
            )
    }

    pub fn overlaps(&self) -> u64 {
        self
            .cells
            .iter()
            .filter(|(_, count)| *count > 1)
            .count()
            as u64
    }

    pub fn draw(&self) {
        let mut w = 0usize;
        let mut h = 0usize;
        for cell in self.cells.iter() {
            if cell.0.x as usize > w {
                w = cell.0.x as usize;
            }
            if cell.0.y as usize > h {
                h = cell.0.y as usize
            }
        }
        let mut arr = Vec::new();
        for y in 0..=h {
            let mut row = Vec::new();
            for _ in 0..=w {
                row.push(0);
            }
            arr.push(row);
        }
        for cell in self.cells.iter() {
            arr[cell.0.y as usize][cell.0.x as usize] = cell.1;
        }
        for y in 0..=h {
            for x in 0..w {
                let val = arr[y][x];
                if val == 0 {
                    print!(".");
                } else {
                    print!("{}", arr[y][x]);
                }
            }
            println!();
        }
    }
}