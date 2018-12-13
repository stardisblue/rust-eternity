#[derive(Debug, Clone)]
pub enum Piece {
    CornerPiece(Props),
    BorderPiece(Props),
    FullPiece(Props),
}

impl Piece {
    pub fn from_vec(id: u8, vec: Vec<u8>) -> Self {
        match vec.as_slice() {
            [0, 0, a, b] => Piece::CornerPiece(Props::new(id, Sides::Corner(*a, *b))),
            [0, a, b, c] => Piece::BorderPiece(Props::new(id, Sides::Border(*a, *b, *c))),
            [a, b, c, d] => Piece::FullPiece(Props::new(id, Sides::Full(*a, *b, *c, *d))),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Props {
    pub id: u8,
    pub kind: Sides,
}

impl Props {
    pub fn new(id: u8, kind: Sides) -> Self {
        Self { id, kind }
    }
}

#[derive(Debug, Clone)]
pub enum Sides {
    Corner(u8, u8),
    Border(u8, u8, u8),
    Full(u8, u8, u8, u8),
}
