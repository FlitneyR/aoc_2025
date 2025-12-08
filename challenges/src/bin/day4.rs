use std::{fmt::{Debug, Display}, str::FromStr};

#[derive(Copy, Clone)]
enum GridCell {
    Empty,
    Roll { accesible: bool },
}

struct Grid {
    cells: Box<[GridCell]>,
    width: usize,
    height: usize,
}

impl GridCell {
    pub fn is_empty(&self) -> bool {
        match self {
            GridCell::Empty => true,
            _ => false,
        }
    }

    pub fn is_accessible(&self) -> bool {
        match self {
            GridCell::Roll { accesible: true } => true,
            _ => false,
        }
    }
}

impl Grid {
    pub fn get_cell(&self, row: usize, col: usize) -> &GridCell {
        &self.cells[col + row * self.width]
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> &mut GridCell {
        &mut self.cells[col + row * self.width]
    }

    pub fn convolve<F>(&mut self, mut f: F)
        where F: FnMut(&mut GridCell, &[GridCell; 8])
    {
        const NEIGHBOUR_OFFSETS: [(isize, isize); 8] = [
            (-1, -1), ( 0, -1), ( 1, -1),
            (-1,  0),           ( 1,  0),
            (-1,  1), ( 0,  1), ( 1,  1),
        ];

        for row in 0..self.height {
            for col in 0..self.width {
                let neighbours: [GridCell; 8] = NEIGHBOUR_OFFSETS
                    .map(|(coff, roff)| {
                        let r = row as isize + roff;
                        let c = col as isize + coff;

                        // this is outside of the grid, return empty
                        if c < 0 || r < 0 || c >= self.width as isize || r >= self.height as isize { GridCell::Empty }
                        // get the cell
                        else { *self.get_cell(r as usize, c as usize) }
                    });

                f(self.get_cell_mut(row, col), &neighbours);
            }
        }
    }

    pub fn update_accessible(&mut self) {
        self.convolve(|cell, neighbours|
            if let GridCell::Roll { accesible } = cell {
                *accesible = neighbours.iter().filter(|cell| !cell.is_empty()).count() < 4;
            }
        );
    }

    // returns the number of removed rolls
    #[allow(unused)]
    pub fn remove_accessible(&mut self) -> usize {
        let mut count_removed = 0;

        for cell in &mut self.cells {
            if cell.is_accessible() { *cell = GridCell::Empty; count_removed += 1 }
        }

        count_removed
    }

    #[allow(unused)]
    pub fn count_accessible(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_accessible()).count()
    }
}

fn main() {
    let mut grid: Grid = aoc_2025_common::get_input().unwrap().collect_to_string().parse().unwrap();

    #[cfg(feature = "part1")]
    {
        #[cfg(feature = "verbose")]
        eprintln!("{grid}");

        grid.update_accessible();

        #[cfg(feature = "verbose")]
        eprintln!("{grid}");

        println!("{}", grid.count_accessible());
    }

    #[cfg(feature = "part2")]
    {
        let mut total_removed = 0;

        loop {
            #[cfg(feature = "verbose")]
            eprintln!("{grid}");

            grid.update_accessible();

            let number_removed = grid.remove_accessible();

            #[cfg(feature = "verbose")]
            eprintln!("Removed {number_removed} rolls");

            if number_removed == 0 { break }
            total_removed += number_removed;
        }

        #[cfg(feature = "verbose")]
        eprintln!("{grid}");

        println!("{total_removed}");
    }
}

// parsing/formatting crap:

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCell::Empty => write!(f, "."),
            GridCell::Roll { accesible: true } => write!(f, "x"),
            GridCell::Roll { accesible: false } => write!(f, "@"),
        }
    }
}

impl Debug for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}


impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                write!(f, "{}", self.cells[index])?;
                index += 1;
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[allow(unused)]
#[derive(Debug)]
enum GridFromStrError {
    UnexpectedCharacter{ index: usize, char: char },
    InconsistentWidth{ row: usize, expected: usize, actual: usize },
    DidntFindEndOfFirstRow,
}

impl FromStr for Grid {
    type Err = GridFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::<GridCell>::new();
        let mut width = Option::<usize>::None;
        let mut row = 0usize;
        let mut col = 0usize;

        for (index, char) in s.chars().enumerate() {
            match char {
                '\n' => {
                    match width {
                        Some(width) => if col != width { return Err(GridFromStrError::InconsistentWidth { row, expected: width, actual: col }) },
                        None => width = Some(col),
                    }

                    col = 0;
                    row += 1;
                    continue;
                },
                '.' => { cells.push(GridCell::Empty) },
                'x' => { cells.push(GridCell::Roll { accesible: true })},
                '@' => { cells.push(GridCell::Roll { accesible: false })},
                 _  => { return Err(GridFromStrError::UnexpectedCharacter { index, char }) },
            }

            col += 1;
        }

        // incase we don't have a trailing \n
        if col != 0 { row += 1 }

        Ok(Self { cells: cells.into(), width: width.ok_or(GridFromStrError::DidntFindEndOfFirstRow)?, height: row  })
    }
}


