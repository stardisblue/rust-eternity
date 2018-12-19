use self::Border::{East, North, South, West};
use board::Compass;
use piece::{Props, Sides};

#[derive(Debug, PartialEq)]
pub enum Cell {
    CornerCell(Option<Props>, (Border, Border)),
    BorderCell(Option<Props>, Border),
    FullCell(Option<Props>, Option<Compass>),
}

impl Cell {
    pub fn get_face(&self, side: Border) -> Face {
        match self {
            Cell::CornerCell(option_prop, borders) => {
                Cell::get_face_corner(side, option_prop, borders)
            }
            Cell::BorderCell(option_prop, border) => {
                Cell::get_face_border(side, option_prop, border)
            }
            Cell::FullCell(
                Some(Props {
                    kind: Sides::Full(a, b, c, d),
                    ..
                }),
                Some(compass),
            ) => Cell::get_face_full(side, a, b, c, d, compass),
            Cell::FullCell(None, None) => Face::None,
            _ => panic!("Bad cell <=> piece allocation"), // No pieces on it :)
        }
    }

    fn get_face_corner(side: Border, props: &Option<Props>, borders: &(Border, Border)) -> Face {
        if !((borders.0 == North || borders.0 == South) && (borders.1 == West || borders.1 == East))
        {
            panic!("wrong border assignations")
        }

        if side == borders.0 || side == borders.1 {
            return Face::Border;
        }

        match props {
            Some(Props {
                kind: Sides::Corner(a, b),
                ..
            }) => match side {
                North => match borders {
                    (South, West) => Face::Color(*a),
                    (South, East) => Face::Color(*b),
                    _ => panic!("something went terribly wrong sir :("),
                },
                East => match borders {
                    (North, West) => Face::Color(*a),
                    (South, West) => Face::Color(*b),
                    _ => panic!("something went terribly wrong sir :("),
                },
                South => match borders {
                    (North, East) => Face::Color(*a),
                    (North, West) => Face::Color(*b),
                    _ => panic!("something went terribly wrong sir :("),
                },
                West => match borders {
                    (South, East) => Face::Color(*a),
                    (North, East) => Face::Color(*b),
                    _ => panic!("something went terribly wrong sir :("),
                },
            },
            None => Face::None,
            _ => panic!("Corner cell does not contain a corner piece"),
        }
    }

    fn get_face_border(side: Border, props: &Option<Props>, border: &Border) -> Face {
        if side == *border {
            return Face::Border;
        }

        match props {
            Some(Props {
                kind: Sides::Border(a, b, c),
                ..
            }) => match side {
                North => match border {
                    East => Face::Color(*c),
                    South => Face::Color(*b),
                    West => Face::Color(*a),
                    _ => unreachable!(),
                },
                East => match border {
                    North => Face::Color(*a),
                    South => Face::Color(*c),
                    West => Face::Color(*b),
                    _ => unreachable!(),
                },
                South => match border {
                    North => Face::Color(*b),
                    East => Face::Color(*a),
                    West => Face::Color(*c),
                    _ => unreachable!(),
                },
                West => match border {
                    North => Face::Color(*c),
                    East => Face::Color(*b),
                    South => Face::Color(*a),
                    _ => unreachable!(),
                },
            },
            None => Face::None,
            _ => panic!("border cell does not contain a border piece"),
        }
    }

    fn get_face_full(side: Border, a: &u8, b: &u8, c: &u8, d: &u8, orientation: &Compass) -> Face {
        match side {
            North => match orientation {
                Compass::North => Face::Color(*a),
                Compass::East => Face::Color(*d),
                Compass::South => Face::Color(*c),
                Compass::West => Face::Color(*b),
            },
            East => match orientation {
                Compass::North => Face::Color(*b),
                Compass::East => Face::Color(*a),
                Compass::South => Face::Color(*d),
                Compass::West => Face::Color(*c),
            },
            South => match orientation {
                Compass::North => Face::Color(*c),
                Compass::East => Face::Color(*b),
                Compass::South => Face::Color(*a),
                Compass::West => Face::Color(*d),
            },
            West => match orientation {
                Compass::North => Face::Color(*d),
                Compass::East => Face::Color(*c),
                Compass::South => Face::Color(*b),
                Compass::West => Face::Color(*a),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Border {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Face {
    Border,
    None,
    Color(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_corner_faces() {
        let corner_cell = Cell::CornerCell(None, (North, West));
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::None, corner_cell.get_face(East));
        assert_eq!(Face::None, corner_cell.get_face(South));

        let corner_cell = Cell::CornerCell(None, (South, West));
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::None, corner_cell.get_face(East));
        assert_eq!(Face::None, corner_cell.get_face(North));

        let corner_cell = Cell::CornerCell(None, (North, East));
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::None, corner_cell.get_face(West));
        assert_eq!(Face::None, corner_cell.get_face(South));

        let corner_cell = Cell::CornerCell(None, (South, East));
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::None, corner_cell.get_face(West));
        assert_eq!(Face::None, corner_cell.get_face(North));
    }

    #[test]
    fn test_piece_corner_face() {
        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 0,
                kind: Sides::Corner(1, 2),
            }),
            (North, West),
        );
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::Color(1), corner_cell.get_face(East));
        assert_eq!(Face::Color(2), corner_cell.get_face(South));

        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 0,
                kind: Sides::Corner(1, 2),
            }),
            (North, East),
        );
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::Color(1), corner_cell.get_face(South));
        assert_eq!(Face::Color(2), corner_cell.get_face(West));

        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 0,
                kind: Sides::Corner(1, 2),
            }),
            (South, West),
        );
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::Color(1), corner_cell.get_face(North));
        assert_eq!(Face::Color(2), corner_cell.get_face(East));

        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 0,
                kind: Sides::Corner(1, 2),
            }),
            (South, East),
        );
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::Color(1), corner_cell.get_face(West));
        assert_eq!(Face::Color(2), corner_cell.get_face(North));
    }

    #[test]
    fn test_empty_border_faces() {
        let border_cell = Cell::BorderCell(None, North);
        assert_eq!(border_cell.get_face(North), Face::Border);
        assert_eq!(border_cell.get_face(East), Face::None);
        assert_eq!(border_cell.get_face(South), Face::None);
        assert_eq!(border_cell.get_face(West), Face::None);

        let border_cell = Cell::BorderCell(None, East);
        assert_eq!(border_cell.get_face(North), Face::None);
        assert_eq!(border_cell.get_face(East), Face::Border);
        assert_eq!(border_cell.get_face(South), Face::None);
        assert_eq!(border_cell.get_face(West), Face::None);

        let border_cell = Cell::BorderCell(None, South);
        assert_eq!(border_cell.get_face(North), Face::None);
        assert_eq!(border_cell.get_face(East), Face::None);
        assert_eq!(border_cell.get_face(South), Face::Border);
        assert_eq!(border_cell.get_face(West), Face::None);

        let border_cell = Cell::BorderCell(None, West);
        assert_eq!(border_cell.get_face(North), Face::None);
        assert_eq!(border_cell.get_face(East), Face::None);
        assert_eq!(border_cell.get_face(South), Face::None);
        assert_eq!(border_cell.get_face(West), Face::Border);
    }

    #[test]
    fn test_piece_border_faces() {
        let border_cell = Cell::BorderCell(
            Some(Props {
                id: 4,
                kind: Sides::Border(1, 2, 3),
            }),
            North,
        );
        assert_eq!(border_cell.get_face(North), Face::Border);
        assert_eq!(border_cell.get_face(East), Face::Color(1));
        assert_eq!(border_cell.get_face(South), Face::Color(2));
        assert_eq!(border_cell.get_face(West), Face::Color(3));

        let border_cell = Cell::BorderCell(
            Some(Props {
                id: 4,
                kind: Sides::Border(1, 2, 3),
            }),
            East,
        );
        assert_eq!(border_cell.get_face(East), Face::Border);
        assert_eq!(border_cell.get_face(South), Face::Color(1));
        assert_eq!(border_cell.get_face(West), Face::Color(2));
        assert_eq!(border_cell.get_face(North), Face::Color(3));

        let border_cell = Cell::BorderCell(
            Some(Props {
                id: 4,
                kind: Sides::Border(1, 2, 3),
            }),
            South,
        );
        assert_eq!(border_cell.get_face(South), Face::Border);
        assert_eq!(border_cell.get_face(West), Face::Color(1));
        assert_eq!(border_cell.get_face(North), Face::Color(2));
        assert_eq!(border_cell.get_face(East), Face::Color(3));

        let border_cell = Cell::BorderCell(
            Some(Props {
                id: 4,
                kind: Sides::Border(1, 2, 3),
            }),
            West,
        );
        assert_eq!(border_cell.get_face(West), Face::Border);
        assert_eq!(border_cell.get_face(North), Face::Color(1));
        assert_eq!(border_cell.get_face(East), Face::Color(2));
        assert_eq!(border_cell.get_face(South), Face::Color(3));
    }

    #[test]
    fn test_empty_full_faces() {
        let full_cell = Cell::FullCell(None, None);
        assert_eq!(full_cell.get_face(North), Face::None);
        assert_eq!(full_cell.get_face(East), Face::None);
        assert_eq!(full_cell.get_face(South), Face::None);
        assert_eq!(full_cell.get_face(West), Face::None);
    }

    #[test]
    fn test_piece_full_faces() {
        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            Some(Compass::North),
        );
        assert_eq!(full_cell.get_face(North), Face::Color(1));
        assert_eq!(full_cell.get_face(East), Face::Color(2));
        assert_eq!(full_cell.get_face(South), Face::Color(3));
        assert_eq!(full_cell.get_face(West), Face::Color(4));
        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            Some(Compass::East),
        );
        assert_eq!(full_cell.get_face(East), Face::Color(1));
        assert_eq!(full_cell.get_face(South), Face::Color(2));
        assert_eq!(full_cell.get_face(West), Face::Color(3));
        assert_eq!(full_cell.get_face(North), Face::Color(4));

        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            Some(Compass::South),
        );
        assert_eq!(full_cell.get_face(South), Face::Color(1));
        assert_eq!(full_cell.get_face(West), Face::Color(2));
        assert_eq!(full_cell.get_face(North), Face::Color(3));
        assert_eq!(full_cell.get_face(East), Face::Color(4));
        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            Some(Compass::West),
        );
        assert_eq!(full_cell.get_face(West), Face::Color(1));
        assert_eq!(full_cell.get_face(North), Face::Color(2));
        assert_eq!(full_cell.get_face(East), Face::Color(3));
        assert_eq!(full_cell.get_face(South), Face::Color(4));
    }

    #[test]
    #[should_panic]
    fn test_corner_piece_panic() {
        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            (North, East),
        );
        corner_cell.get_face(South);
    }
    #[test]
    #[should_panic]
    fn test_corner_border_panic() {
        let corner_cell = Cell::CornerCell(
            Some(Props {
                id: 12,
                kind: Sides::Corner(1, 2),
            }),
            (North, South),
        );
        corner_cell.get_face(South);
    }

    #[test]
    #[should_panic]
    fn test_border_piece_panic() {
        let border_cell = Cell::BorderCell(
            Some(Props {
                id: 12,
                kind: Sides::Full(1, 2, 3, 4),
            }),
            North,
        );
        border_cell.get_face(South);
    }

    #[test]
    #[should_panic]
    fn test_full_piece_panic() {
        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Border(1, 2, 3),
            }),
            Some(Compass::North),
        );
        full_cell.get_face(South);
    }

    #[test]
    #[should_panic]
    fn test_full_compass_missing_panic() {
        let full_cell = Cell::FullCell(
            Some(Props {
                id: 12,
                kind: Sides::Border(1, 2, 3),
            }),
            None,
        );
        full_cell.get_face(South);
    }

    #[test]
    #[should_panic]
    fn test_full_no_piece_compass_panic() {
        let full_cell = Cell::FullCell(None, Some(Compass::North));
        full_cell.get_face(South);
    }
}
