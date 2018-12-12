#[derive(Debug)]
pub enum Piece {
    CornerPiece(Props<Corner>),
    BorderPiece(Props<Border>),
    FullPiece(Props<Full>),
}

#[derive(Debug, Clone)]
pub struct Props<T> {
    id: u8,
    kind: T,
}

#[derive(Debug, Clone)]
pub struct Corner(u8, u8);

#[derive(Debug, Clone)]
pub struct Border(u8, u8, u8);

#[derive(Debug, Clone)]
pub struct Full(u8, u8, u8, u8);

impl Piece {
    pub fn from_vec(id: u8, vec: Vec<u8>) -> Self {
        match vec.as_slice() {
            [0, 0, a, b] => Piece::CornerPiece(Props {
                id,
                kind: Corner(*a, *b),
            }),
            [0, a, b, c] => Piece::BorderPiece(Props {
                id,
                kind: Border(*a, *b, *c),
            }),
            [a, b, c, d] => Piece::FullPiece(Props {
                id,
                kind: Full(*a, *b, *c, *d),
            }),
            _ => unreachable!(),
        }
    }
}
