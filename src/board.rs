use std::collections::HashSet;

pub enum Rotate {
    CounterClockWise,
    ClockWise,
    Half,
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
    pub id: u8,
    sides: (u8, u8, u8, u8),
    orientation: Orientation,
}

impl BoardPiece {
    pub fn from_vec(id: u8, vec: Vec<u8>) -> BoardPiece {
        BoardPiece {
            id,
            sides: (vec[0], vec[1], vec[2], vec[3]),
            orientation: Orientation::Top,
        }
    }

    pub fn rotate(&mut self, direction: Rotate) {
        let (top, right, bot, left) = self.sides;

        match direction {
            Rotate::CounterClockWise => {
                self.sides = (right, bot, left, top);
                self.orientation = match self.orientation {
                    Orientation::Top => Orientation::Right,
                    Orientation::Right => Orientation::Bottom,
                    Orientation::Bottom => Orientation::Left,
                    Orientation::Left => Orientation::Top,
                };
            }
            Rotate::ClockWise => {
                self.sides = (left, top, right, bot);
                self.orientation = match self.orientation {
                    Orientation::Top => Orientation::Left,
                    Orientation::Right => Orientation::Top,
                    Orientation::Bottom => Orientation::Right,
                    Orientation::Left => Orientation::Bottom,
                };
            }
            Rotate::Half => {
                self.sides = (bot, left, top, right);
                self.orientation = match self.orientation {
                    Orientation::Top => Orientation::Bottom,
                    Orientation::Right => Orientation::Left,
                    Orientation::Bottom => Orientation::Top,
                    Orientation::Left => Orientation::Right,
                };
            }
        }
    }
}

#[derive(Debug)]
pub struct BoardCell {
    pos: (u8, u8),
    contains: Option<u8>,
}

impl BoardCell {
    pub fn put(&mut self, piece: u8) {
        self.contains = Some(piece)
    }
}

#[derive(Debug)]
pub struct BoardGame {
    pub size: u8,
    pub placed: Vec<bool>,
    pub colors: Vec<HashSet<u8>>,
    pub cells: Vec<Vec<BoardCell>>,
    pub pieces: Vec<BoardPiece>,
}

impl BoardGame {
    pub fn new(content: Vec<String>) -> BoardGame {
        // println!("{:?}", &content[4..]);

        let size = content[0].parse::<u8>().expect("must be a number");
        let cols = content[1].parse::<u8>().expect("must be a number");
        let pieces = content[4..]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                BoardPiece::from_vec(
                    i as u8,
                    line.split_whitespace()
                        .map(|nb| nb.parse::<u8>().expect("must be a number"))
                        .collect(),
                )
            }).collect();

        let colors = BoardGame::index(&pieces, cols);
        let placed = vec![false; (size as usize) * (size as usize)];
        BoardGame {
            size,
            pieces,
            colors,
            placed,
            cells: (0..size)
                .map(|x| {
                    (0..size)
                        .map(|y| BoardCell {
                            pos: (x, y),
                            contains: None,
                        }).collect()
                }).collect(),
        }
    }

    /**
     *
     */
    fn index(pieces: &Vec<BoardPiece>, cols: u8) -> Vec<HashSet<u8>> {
        let mut colors_index: Vec<HashSet<u8>> = vec![HashSet::new(); cols as usize + 1];

        for piece in pieces {
            BoardGame::add_to_color_index(&mut colors_index, piece.sides.0, piece.id);
            BoardGame::add_to_color_index(&mut colors_index, piece.sides.1, piece.id);
            BoardGame::add_to_color_index(&mut colors_index, piece.sides.2, piece.id);
            BoardGame::add_to_color_index(&mut colors_index, piece.sides.3, piece.id);
        }

        colors_index
    }

    fn add_to_color_index(colors_index: &mut Vec<HashSet<u8>>, color: u8, piece_id: u8) {
        match colors_index.get_mut(color as usize) {
            Some(x) => {
                x.insert(piece_id);
            }
            None => panic!("this is not normal"),
        }
    }
    pub fn find_placable_pieces(&self, x: u8, y: u8) -> HashSet<u8> {
        let (top, right, bottom, left) = self.get_neighbor_colors(x, y);
        let mut filtered: HashSet<_> = self
            .placed
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, is_placed)| if !is_placed { Some(i as u8) } else { None })
            .collect();

        for side in vec![top, right, bottom, left] {
            filtered = match side {
                Some(i) => filtered
                    .intersection(&self.colors[i as usize])
                    .map(|&x| x)
                    .collect(),
                None => filtered,
            };
        }

        filtered
    }

    fn get_neighbor_colors(
        &self,
        x: u8,
        y: u8,
    ) -> (Option<u8>, Option<u8>, Option<u8>, Option<u8>) {
        let top = if x == 0 {
            Some(0)
        } else {
            match self.cells[x as usize - 1][y as usize].contains {
                Some(id) => Some(self.pieces[id as usize].sides.2),
                None => None,
            }
        };

        let right = if y == self.size - 1 {
            Some(0)
        } else {
            match self.cells[x as usize][y as usize + 1].contains {
                Some(id) => Some(self.pieces[id as usize].sides.3),
                None => None,
            }
        };

        let bottom = if x == self.size - 1 {
            Some(0)
        } else {
            match self.cells[x as usize + 1][y as usize].contains {
                Some(id) => Some(self.pieces[id as usize].sides.0),
                None => None,
            }
        };
        let left = if y == 0 {
            Some(0)
        } else {
            match self.cells[x as usize][y as usize - 1].contains {
                Some(id) => Some(self.pieces[id as usize].sides.1),
                None => None,
            }
        };

        (top, right, bottom, left)
    }
}
