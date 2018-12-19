use piece;
use piece::Piece;
use cell::{Cell, Border, Face};

#[derive(Debug)]
pub struct BoardGame {
    pub size: u8,
    pub pieces: Vec<Piece>,
    pub placed: Vec<bool>,
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq)]
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

        use cell::Border::{East, North, South, West};

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

#[cfg(test)]
mod tests {
    use super::*;
    fn create_board() -> BoardGame {
        let file_content = "4\n5\n1\n1 1 1 1\n0 0 1 1\n0 0 1 2\n0 0 2 1\n0 0 2 2\n0 1 3 1\n0 1 3 2\n0 1 4 1\n0 1 5 2\n0 2 4 1\n0 2 4 2\n0 2 5 1\n0 2 5 2\n3 3 5 5\n3 4 3 5\n3 4 4 4\n3 5 5 4".to_string();

        BoardGame::new(file_content.lines().map(|line| line.to_string()).collect())
    }

    #[test]
    fn test_create_board() {
        let board = tests::create_board();
        let last: usize = (board.size - 1) as usize;

        assert_eq!(board.size, 4);
        assert_eq!(board.pieces.len(), 16);
        assert_eq!(board.cells.len(), 4);

        assert_eq!(
            board.cells[0][0],
            Cell::CornerCell(None, (Border::North, Border::West))
        );
        assert_eq!(
            board.cells[0][last],
            Cell::CornerCell(None, (Border::North, Border::East))
        );
        assert_eq!(
            board.cells[last][0],
            Cell::CornerCell(None, (Border::South, Border::West))
        );
        assert_eq!(
            board.cells[last][last],
            Cell::CornerCell(None, (Border::South, Border::East))
        );

        // Borders
        for i in 1..(last - 1) {
            assert_eq!(board.cells[0][i], Cell::BorderCell(None, Border::North));
            assert_eq!(board.cells[i][0], Cell::BorderCell(None, Border::West));
            assert_eq!(board.cells[last][i], Cell::BorderCell(None, Border::South));
            assert_eq!(board.cells[i][last], Cell::BorderCell(None, Border::East));
        }

        //Center

        for i in 1..(last - 1) {
            for j in 1..(last - 1) {
                assert_eq!(board.cells[i][j], Cell::FullCell(None, None));
            }
        }

        // #Pieces
        use piece::Piece;
        assert_eq!(
            board.pieces,
            vec![
                Piece::from_vec(0, vec![0, 0, 1, 1]),
                Piece::from_vec(1, vec![0, 0, 1, 2]),
                Piece::from_vec(2, vec![0, 0, 2, 1]),
                Piece::from_vec(3, vec![0, 0, 2, 2]),
                Piece::from_vec(4, vec![0, 1, 3, 1]),
                Piece::from_vec(5, vec![0, 1, 3, 2]),
                Piece::from_vec(6, vec![0, 1, 4, 1]),
                Piece::from_vec(7, vec![0, 1, 5, 2]),
                Piece::from_vec(8, vec![0, 2, 4, 1]),
                Piece::from_vec(9, vec![0, 2, 4, 2]),
                Piece::from_vec(10, vec![0, 2, 5, 1]),
                Piece::from_vec(11, vec![0, 2, 5, 2]),
                Piece::from_vec(12, vec![3, 3, 5, 5]),
                Piece::from_vec(13, vec![3, 4, 3, 5]),
                Piece::from_vec(14, vec![3, 4, 4, 4]),
                Piece::from_vec(15, vec![3, 5, 5, 4]),

            ]
        );
    }
}
