use piece;
use piece::Piece;

#[derive(Debug)]
pub struct BoardGame {
    pub pieces: Vec<Piece>,
    pub placed: Vec<bool>,
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub enum Compass {
    North,
    East,
    South,
    West,
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

        let last_index = size - 1;

        use Borders::{East, North, South, West};

        let mut cells = Vec::with_capacity(size as usize);
        for x in 0..size {
            let mut row = Vec::with_capacity(size as usize);
            for y in 0..size {
                row.push(
                    if (x == 0 || x == last_index) && (y == 0 || y == last_index) {
                        Cell::CornerCell(
                            None,
                            (
                                if x == 0 { North } else { South },
                                if y == 0 { West } else { East },
                            ),
                        )
                    } else if x == 0 || x == last_index || y == 0 || y == last_index {
                        Cell::BorderCell(
                            None,
                            if x == 0 {
                                North
                            } else if x == last_index {
                                South
                            } else if y == 0 {
                                West
                            } else if y == last_index {
                                East
                            } else {
                                unreachable!()
                            },
                        )
                    } else {
                        Cell::FullCell(None, None)
                    },
                );
            }
            cells.push(row);
        }

        Self {
            pieces,
            placed,
            cells,
        }
    }

    pub fn put_piece(&mut self, index: u8, pos: (u8, u8), compass: Option<Compass>) {
        if self.placed[index as usize] {
            panic!("this piece ({}) is already placed ", index)
        }

        let (x, y) = pos;
        let piece = self.pieces[index as usize].clone();
        self.placed[index as usize] = true;
        match self.cells[y as usize][x as usize] {
            Cell::CornerCell(ref mut a, (_, _)) => match a {
                Some(_) => panic!("already a piece placed at ({},{})", x, y),
                None => match piece {
                    Piece::FullPiece(_) => panic!("cannot put full piece on corner cell"),
                    Piece::BorderPiece(_) => panic!("cannot put border piece on corner cell"),

                    Piece::CornerPiece(props) => *a = Some(props),
                },
            },
            Cell::BorderCell(ref mut a, _) => match a {
                Some(_) => panic!("already a piece placed at ({},{})", x, y),
                None => match piece {
                    Piece::CornerPiece(_) => panic!("cannot put corner piece on border cell"),
                    Piece::FullPiece(_) => panic!("cannot put full piece on border cell"),

                    Piece::BorderPiece(props) => *a = Some(props),
                },
            },
            Cell::FullCell(ref mut a, ref mut facing) => match a {
                Some(_) => panic!("already a piece placed at ({},{})", x, y),
                None => match piece {
                    Piece::CornerPiece(_) => panic!("cannot put corner piece on full cell"),
                    Piece::BorderPiece(_) => panic!("cannot put border piece on full cell"),

                    Piece::FullPiece(props) => {
                        *a = Some(props);
                        *facing = match compass {
                            None => panic!("no facing specified for full piece placement"),
                            _ => compass,
                        };
                    }
                },
            },
        }
    }

    pub fn remove_piece(&mut self, pos: (u8, u8)) {
        let (x, y) = pos;
        let p = match self.cells[y as usize][x as usize] {
            Cell::CornerCell(ref mut a, _) => a,
            Cell::BorderCell(ref mut a, _) => a,
            Cell::FullCell(ref mut a, ref mut compass) => {
                *compass = None;
                a
            }
        };

        match p {
            Some(piece::Props { id, .. }) => self.placed[*id as usize] = false,
            None => panic!("cannot remove piece from empty cell"),
        };

        *p = None;
    }
}

#[derive(Debug)]
pub enum Cell {
    CornerCell(Option<piece::Props>, (Borders, Borders)),
    BorderCell(Option<piece::Props>, Borders),
    FullCell(Option<piece::Props>, Option<Compass>),
}

#[derive(Debug)]
pub enum Borders {
    North,
    East,
    South,
    West,
}
