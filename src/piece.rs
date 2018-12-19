#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    CornerPiece(Props),
    BorderPiece(Props),
    FullPiece(Props),
}

impl Piece {
    pub fn from_vec(id: u8, vec: Vec<u8>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_create_panic() {
        Piece::from_vec(1, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_piece_corner() {
        assert_eq!(
            Piece::from_vec(1, vec![0, 0, 1, 2]),
            Piece::CornerPiece(Props::new(1, Sides::Corner(1, 2))),
        );
    }

    #[test]
    fn test_piece_border() {
        assert_eq!(
            Piece::from_vec(1, vec![0, 1, 2, 3]),
            Piece::BorderPiece(Props::new(1, Sides::Border(1, 2, 3)))
        );
    }

    #[test]
    fn test_piece_full() {
        assert_eq!(
            Piece::from_vec(1, vec![1, 2, 3, 4]),
            Piece::FullPiece(Props::new(1, Sides::Full(1, 2, 3, 4)))
        );
    }
}
