pub enum Shift {
    Left,
    Right,
}

#[derive(Debug)]
enum Orientation {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct BoardPiece {
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
    orientation: Orientation,
}

#[derive(Debug)]
pub struct BoardCell {
    x: u8,
    y: u8,
    contains: Option<u8>,
}

impl BoardPiece {
    pub fn shift(self, direction: Shift) -> BoardPiece {
        match direction {
            Shift::Left => BoardPiece {
                top: self.right,
                right: self.bottom,
                bottom: self.left,
                left: self.top,
                orientation: match self.orientation {
                    Orientation::Top => Orientation::Right,
                    Orientation::Right => Orientation::Bottom,
                    Orientation::Bottom => Orientation::Left,
                    Orientation::Left => Orientation::Top,
                },
            },
            Shift::Right => BoardPiece {
                top: self.left,
                right: self.top,
                bottom: self.right,
                left: self.bottom,
                orientation: match self.orientation {
                    Orientation::Top => Orientation::Left,
                    Orientation::Right => Orientation::Top,
                    Orientation::Bottom => Orientation::Right,
                    Orientation::Left => Orientation::Bottom,
                },
            },
        }
    }
}

#[derive(Debug)]
pub struct BoardGame {
    cells: Vec<BoardCell>,
    pieces: Vec<BoardPiece>,
}

impl BoardGame {
    pub fn new(content: Vec<String>) /*-> BoardGame*/
    {
        // println!("{:?}", &content[4..]);

        let numbers = &content[4..]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|nb| nb.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            }).collect::<Vec<Vec<u8>>>();
        println!("{:?}", numbers);

        // let mut cells = Vec::with_capacity((size as usize) * (size as usize));

        // for x in 0..size {
        //     for y in 0..size {
        //         cells.push(BoardCell {
        //             x: x,
        //             y: y,
        //             contains: None,
        //         });
        //     }
        // }

        // BoardGame {
        //     cells: cells,
        //     pieces: Vec::with_capacity((size as usize) * (size as usize)),
        // }
    }
}
