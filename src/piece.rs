#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    CornerPiece(Props),
    BorderPiece(Props),
    FullPiece(Props),
}

impl Piece {
    pub fn new(id: u8, vec: Vec<u8>) -> Self {
        match vec.as_slice() {
            [0, 0, a, b] if *a > 0 && *b > 0 => {
                Piece::CornerPiece(Props::new(id, Sides::Corner(*a, *b)))
            }
            [0, a, b, c] if *b > 0 && *c > 0 => {
                Piece::BorderPiece(Props::new(id, Sides::Border(*a, *b, *c)))
            }
            [a, b, c, d] if *c > 0 && *d > 0 => {
                Piece::FullPiece(Props::new(id, Sides::Full(*a, *b, *c, *d)))
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Props {
    pub id: u8,
    pub kind: Sides,
}

impl Props {
    pub fn new(id: u8, kind: Sides) -> Self {
        Self { id, kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sides {
    Corner(u8, u8),
    Border(u8, u8, u8),
    Full(u8, u8, u8, u8),
}

use board::Compass;
use cell::{Border, Face};

impl Sides {
    pub fn get_corner_offset(borders: &(Border, Border)) -> &Compass {
        match borders {
            (Border::North, Border::West) => &Compass::East,
            (Border::North, Border::East) => &Compass::South,
            (Border::South, Border::East) => &Compass::West,
            (Border::South, Border::West) => &Compass::North,
            _ => panic!("Not a correct borders for corner"),
        }
    }

    pub fn get_border_offset(border: &Border) -> &Compass {
        match border {
            Border::North => &Compass::East,
            Border::East => &Compass::South,
            Border::South => &Compass::West,
            Border::West => &Compass::North,
        }
    }

    pub fn get_faces(&self, offset: &Compass) -> (Face, Face, Face, Face) {
        match self {
            Sides::Corner(a, b) => Sides::get_faces_corner(offset, a, b),
            Sides::Border(a, b, c) => Sides::get_faces_border(offset, a, b, c),
            Sides::Full(a, b, c, d) => Sides::get_faces_full(offset, a, b, c, d),
        }
    }

    pub fn get_faces_corner(offset: &Compass, a: &u8, b: &u8) -> (Face, Face, Face, Face) {
        match offset {
            Compass::North => (Face::Color(*a), Face::Color(*b), Face::Border, Face::Border),
            Compass::East => (Face::Border, Face::Color(*a), Face::Color(*b), Face::Border),
            Compass::South => (Face::Border, Face::Border, Face::Color(*a), Face::Color(*b)),
            Compass::West => (Face::Color(*b), Face::Border, Face::Border, Face::Color(*a)),
        }
    }

    pub fn get_faces_border(offset: &Compass, a: &u8, b: &u8, c: &u8) -> (Face, Face, Face, Face) {
        match offset {
            Compass::North => (
                Face::Color(*a),
                Face::Color(*b),
                Face::Color(*c),
                Face::Border,
            ),
            Compass::East => (
                Face::Border,
                Face::Color(*a),
                Face::Color(*b),
                Face::Color(*c),
            ),
            Compass::South => (
                Face::Color(*c),
                Face::Border,
                Face::Color(*a),
                Face::Color(*b),
            ),
            Compass::West => (
                Face::Color(*b),
                Face::Color(*c),
                Face::Border,
                Face::Color(*a),
            ),
        }
    }

    pub fn get_faces_full(
        offset: &Compass,
        a: &u8,
        b: &u8,
        c: &u8,
        d: &u8,
    ) -> (Face, Face, Face, Face) {
        match offset {
            Compass::North => (
                Face::Color(*a),
                Face::Color(*b),
                Face::Color(*c),
                Face::Color(*d),
            ),
            Compass::East => (
                Face::Color(*d),
                Face::Color(*a),
                Face::Color(*b),
                Face::Color(*c),
            ),
            Compass::South => (
                Face::Color(*c),
                Face::Color(*d),
                Face::Color(*a),
                Face::Color(*b),
            ),
            Compass::West => (
                Face::Color(*b),
                Face::Color(*c),
                Face::Color(*d),
                Face::Color(*a),
            ),
        }
    }

    pub fn get_face(&self, face: Border, offset: &Compass) -> Face {
        match self {
            Sides::Corner(a, b) => Sides::get_face_corner(face, offset, a, b),
            Sides::Border(a, b, c) => Sides::get_face_border(face, offset, a, b, c),
            Sides::Full(a, b, c, d) => Sides::get_face_full(face, offset, a, b, c, d),
        }
    }

    pub fn get_face_corner(face: Border, offset: &Compass, a: &u8, b: &u8) -> Face {
        match offset {
            Compass::North => match face {
                Border::North => Face::Color(*a),
                Border::East => Face::Color(*b),
                _ => Face::Border,
            },
            Compass::East => match face {
                Border::East => Face::Color(*a),
                Border::South => Face::Color(*b),
                _ => Face::Border,
            },
            Compass::South => match face {
                Border::South => Face::Color(*a),
                Border::West => Face::Color(*b),
                _ => Face::Border,
            },
            Compass::West => match face {
                Border::West => Face::Color(*a),
                Border::North => Face::Color(*b),
                _ => Face::Border,
            },
        }
    }

    pub fn get_face_border(face: Border, offset: &Compass, a: &u8, b: &u8, c: &u8) -> Face {
        match offset {
            Compass::North => match face {
                Border::North => Face::Color(*a),
                Border::East => Face::Color(*b),
                Border::South => Face::Color(*c),
                _ => Face::Border,
            },
            Compass::East => match face {
                Border::East => Face::Color(*a),
                Border::South => Face::Color(*b),
                Border::West => Face::Color(*c),
                _ => Face::Border,
            },
            Compass::South => match face {
                Border::South => Face::Color(*a),
                Border::West => Face::Color(*b),
                Border::North => Face::Color(*c),
                _ => Face::Border,
            },
            Compass::West => match face {
                Border::West => Face::Color(*a),
                Border::North => Face::Color(*b),
                Border::East => Face::Color(*c),
                _ => Face::Border,
            },
        }
    }

    pub fn get_face_full(face: Border, offset: &Compass, a: &u8, b: &u8, c: &u8, d: &u8) -> Face {
        match offset {
            Compass::North => match face {
                Border::North => Face::Color(*a),
                Border::East => Face::Color(*b),
                Border::South => Face::Color(*c),
                Border::West => Face::Color(*d),
            },
            Compass::East => match face {
                Border::East => Face::Color(*a),
                Border::South => Face::Color(*b),
                Border::West => Face::Color(*c),
                Border::North => Face::Color(*d),
            },
            Compass::South => match face {
                Border::South => Face::Color(*a),
                Border::West => Face::Color(*b),
                Border::North => Face::Color(*c),
                Border::East => Face::Color(*d),
            },
            Compass::West => match face {
                Border::West => Face::Color(*a),
                Border::North => Face::Color(*b),
                Border::East => Face::Color(*c),
                Border::South => Face::Color(*d),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_create_panic() {
        Piece::new(1, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_piece_corner() {
        assert_eq!(
            Piece::new(1, vec![0, 0, 1, 2]),
            Piece::CornerPiece(Props::new(1, Sides::Corner(1, 2))),
        );
    }

    #[test]
    fn test_piece_border() {
        assert_eq!(
            Piece::new(1, vec![0, 1, 2, 3]),
            Piece::BorderPiece(Props::new(1, Sides::Border(1, 2, 3)))
        );
    }

    #[test]
    fn test_piece_full() {
        assert_eq!(
            Piece::new(1, vec![1, 2, 3, 4]),
            Piece::FullPiece(Props::new(1, Sides::Full(1, 2, 3, 4)))
        );
    }
}
