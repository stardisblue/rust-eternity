use piece;
use piece::Piece;

#[derive(Debug)]
pub struct BoardGame {
    pieces: Vec<Piece>,
    placed: Vec<bool>,
    cells: Vec<Vec<Option<Cell>>>,
}

impl BoardGame {
    pub fn new(content: Vec<String>) -> Self {
        // println!("{:?}", &content[4..]);

        let size = content[0].parse::<u8>().expect("must be a number");
        let pieces = content[4..]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                Piece::from_vec(
                    i as u8,
                    line.split_whitespace()
                        .map(|nb| nb.parse::<u8>().expect("must be a number"))
                        .collect(),
                )
            }).collect();

        let placed = vec![false; (size as usize) * (size as usize)];

        let mut cells = vec![vec![None; size as usize]; size as usize];
        let last_index = size - 1;

        for i in 0..size {
            for j in 0..size {
                match (i, j) {
                    (i, j) if (i == 0 || i == last_index) && (j == 0 || j == last_index) => {
                        cells[i as usize][j as usize] = Some(Cell::CornerCell(None));
                    }
                    (i, j) if i == 0 || i == last_index || j == 0 || j == last_index => {
                        cells[i as usize][j as usize] = Some(Cell::BorderCell(None));
                    }
                    (i, j) => {
                        cells[i as usize][j as usize] = Some(Cell::FullCell(None));
                    }
                };
            }
        }

        Self {
            pieces,
            placed,
            cells,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    CornerCell(Option<piece::Props<piece::Corner>>),
    BorderCell(Option<piece::Props<piece::Border>>),
    FullCell(Option<piece::Props<piece::Full>>),
}
