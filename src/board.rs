use piece;
use piece::Piece;

#[derive(Debug)]
pub struct BoardGame {
    pub size: u8,
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
            })
            .collect();

        let placed = vec![false; (size as usize) * (size as usize)];

        let last_index = size - 1;

        use Border::{East, North, South, West};

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
            size,
            pieces,
            placed,
            cells,
        }
    }

    pub fn get_frontier(&self, pos: (u8, u8)) -> (Face, Face, Face, Face) {
        let (x, y) = pos;
        let last_index = self.size - 1;
        (
            match x {
                0 => Face::Border,
                _ => self.cells[y as usize][(x - 1) as usize].get_face(Border::South),
            },
            match y {
                a if a == last_index => Face::Border,
                _ => self.cells[(y + 1) as usize][x as usize].get_face(Border::West),
            },
            match x {
                a if a == last_index => Face::Border,
                _ => self.cells[y as usize][(x + 1) as usize].get_face(Border::North),
            },
            match y {
                0 => Face::Border,
                _ => self.cells[(y - 1) as usize][x as usize].get_face(Border::East),
            },
        )
    }

    pub fn put_piece(&mut self, index: u8, pos: (u8, u8), compass: Option<Compass>) {
        if self.placed[index as usize] {
            panic!("this piece ({}) is already placed ", index)
        }

        let (x, y) = pos;
        let piece = self.pieces[index as usize].clone();
        self.placed[index as usize] = true;
        match self.cells[y as usize][x as usize] {
            Cell::CornerCell(ref mut a, _) => match a {
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
    CornerCell(Option<piece::Props>, (Border, Border)),
    BorderCell(Option<piece::Props>, Border),
    FullCell(Option<piece::Props>, Option<Compass>),
}

impl Cell {
    pub fn get_face(&self, side: Border) -> Face {
        use piece::{Props, Sides};
        match self {
            Cell::CornerCell(
                Some(Props {
                    kind: Sides::Corner(a, b),
                    ..
                }),
                borders,
            ) => Cell::get_face_corner(side, a, b, borders),
            Cell::BorderCell(
                Some(Props {
                    kind: Sides::Border(a, b, c),
                    ..
                }),
                border,
            ) => Cell::get_face_border(side, a, b, c, border),
            Cell::FullCell(
                Some(Props {
                    kind: Sides::Full(a, b, c, d),
                    ..
                }),
                Some(compass),
            ) => Cell::get_face_full(side, a, b, c, d, compass),
            _ => Face::None, // No pieces on it :)
        }
    }

    fn get_face_corner(side: Border, a: &u8, b: &u8, borders: &(Border, Border)) -> Face {
        match side {
            Border::North => match borders {
                (Border::North, _) => Face::Border,
                (Border::South, Border::West) => Face::Color(*a),
                (Border::South, Border::East) => Face::Color(*b),
                _ => panic!("something went terribly wrong sir :("),
            },
            Border::East => match borders {
                (_, Border::East) => Face::Border,
                (Border::North, Border::West) => Face::Color(*a),
                (Border::South, Border::West) => Face::Color(*b),
                _ => panic!("something went terribly wrong sir :("),
            },
            Border::South => match borders {
                (Border::South, _) => Face::Border,
                (Border::North, Border::East) => Face::Color(*a),
                (Border::North, Border::West) => Face::Color(*b),
                _ => panic!("something went terribly wrong sir :("),
            },
            Border::West => match borders {
                (_, Border::West) => Face::Border,
                (Border::South, Border::East) => Face::Color(*a),
                (Border::North, Border::East) => Face::Color(*b),
                _ => panic!("something went terribly wrong sir :("),
            },
        }
    }

    fn get_face_border(side: Border, a: &u8, b: &u8, c: &u8, border: &Border) -> Face {
        match side {
            Border::North => match border {
                Border::North => Face::Border,
                Border::East => Face::Color(*c),
                Border::South => Face::Color(*b),
                Border::West => Face::Color(*a),
            },
            Border::East => match border {
                Border::North => Face::Color(*a),
                Border::East => Face::Border,
                Border::South => Face::Color(*c),
                Border::West => Face::Color(*b),
            },
            Border::South => match border {
                Border::North => Face::Color(*b),
                Border::East => Face::Color(*a),
                Border::South => Face::Border,
                Border::West => Face::Color(*c),
            },
            Border::West => match border {
                Border::North => Face::Color(*c),
                Border::East => Face::Color(*b),
                Border::South => Face::Color(*a),
                Border::West => Face::Border,
            },
        }
    }

    fn get_face_full(side: Border, a: &u8, b: &u8, c: &u8, d: &u8, orientation: &Compass) -> Face {
        match side {
            Border::North => match orientation {
                Compass::North => Face::Color(*a),
                Compass::East => Face::Color(*d),
                Compass::South => Face::Color(*c),
                Compass::West => Face::Color(*b),
            },
            Border::East => match orientation {
                Compass::North => Face::Color(*b),
                Compass::East => Face::Color(*a),
                Compass::South => Face::Color(*d),
                Compass::West => Face::Color(*c),
            },
            Border::South => match orientation {
                Compass::North => Face::Color(*c),
                Compass::East => Face::Color(*b),
                Compass::South => Face::Color(*a),
                Compass::West => Face::Color(*d),
            },
            Border::West => match orientation {
                Compass::North => Face::Color(*d),
                Compass::East => Face::Color(*c),
                Compass::South => Face::Color(*b),
                Compass::West => Face::Color(*a),
            },
        }
    }
}

#[derive(Debug)]
pub enum Border {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub enum Face {
    Border,
    None,
    Color(u8),
}

#[cfg(test)]
mod tests {
    use BoardGame;
    #[test]
    fn create_board() {
        let file_content = "4\n5\n1\n1 1 1 1\n0 0 1 1\n0 0 1 2\n0 0 2 1\n0 0 2 2\n0 1 3 1\n0 1 3 2\n0 1 4 1\n0 1 5 2\n0 2 4 1\n0 2 4 2\n0 2 5 1\n0 2 5 2\n3 3 5 5\n3 4 3 5\n3 4 4 4\n3 5 5 4".to_string();

        let board = BoardGame::new(file_content.lines().map(|line| line.to_string()).collect());
        unimplemented!()
    }
}
