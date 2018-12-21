use board::Compass;
use piece::{Props, Sides};

#[derive(Debug, PartialEq)]
pub enum Cell {
    CornerCell(Option<Props>, (Border, Border)),
    BorderCell(Option<Props>, Border),
    FullCell(Option<Props>, Option<Compass>),
}

impl Cell {
    pub fn get_faces(&self) -> (Face, Face, Face, Face) {
        match self {
            Cell::CornerCell(props, borders) => Cell::get_faces_corner(props, borders),
            Cell::BorderCell(props, border) => Cell::get_faces_border(props, border),
            Cell::FullCell(props, compass) => Cell::get_faces_full(props, compass),
        }
    }

    fn get_faces_corner(
        props: &Option<Props>,
        borders: &(Border, Border),
    ) -> (Face, Face, Face, Face) {
        use self::Border::{East, North, South, West};

        let compass = Sides::get_corner_offset(borders);

        match props {
            Some(Props {
                kind: Sides::Corner(a, b),
                ..
            }) => Sides::get_faces_corner(compass, a, b),
            None => {
                let (ns, we) = borders;
                (
                    if *ns == North {
                        Face::Border
                    } else {
                        Face::None
                    },
                    if *we == East {
                        Face::Border
                    } else {
                        Face::None
                    },
                    if *ns == South {
                        Face::Border
                    } else {
                        Face::None
                    },
                    if *we == West {
                        Face::Border
                    } else {
                        Face::None
                    },
                )
            }
            _ => panic!("piece is not matching corner cell"),
        }
    }

    fn get_faces_border(props: &Option<Props>, border: &Border) -> (Face, Face, Face, Face) {
        use self::Border::{East, North, South, West};

        let compass = Sides::get_border_offset(border);

        match props {
            Some(Props {
                kind: Sides::Border(a, b, c),
                ..
            }) => Sides::get_faces_border(compass, a, b, c),
            None => (
                if *border == North {
                    Face::Border
                } else {
                    Face::None
                },
                if *border == East {
                    Face::Border
                } else {
                    Face::None
                },
                if *border == South {
                    Face::Border
                } else {
                    Face::None
                },
                if *border == West {
                    Face::Border
                } else {
                    Face::None
                },
            ),
            _ => panic!("piece cannot be placed on corner cell"),
        }
    }

    fn get_faces_full(
        props: &Option<Props>,
        compass: &Option<Compass>,
    ) -> (Face, Face, Face, Face) {
        match compass {
            Some(compass) => match props {
                Some(Props {
                    kind: Sides::Full(a, b, c, d),
                    ..
                }) => Sides::get_faces_full(compass, a, b, c, d),
                _ => panic!("Not a full piece"),
            },
            None => match props {
                None => (Face::None, Face::None, Face::None, Face::None),
                _ => panic!("doesn't have a compass"),
            },
        }
    }

    pub fn get_face(&self, side: Border) -> Face {
        match self {
            Cell::CornerCell(props, borders) => Cell::get_face_corner(side, props, borders),
            Cell::BorderCell(props, border) => Cell::get_face_border(side, props, border),
            Cell::FullCell(props, compass) => Cell::get_face_full(side, props, compass),
        }
    }

    fn get_face_corner(side: Border, props: &Option<Props>, borders: &(Border, Border)) -> Face {
        let offset = Sides::get_corner_offset(borders);

        if side == borders.0 || side == borders.1 {
            return Face::Border;
        }

        match props {
            Some(Props {
                kind: Sides::Corner(a, b),
                ..
            }) => Sides::get_face_corner(side, offset, a, b),
            None => Face::None,
            _ => unreachable!(),
        }
    }

    fn get_face_border(side: Border, props: &Option<Props>, border: &Border) -> Face {
        let offset = Sides::get_border_offset(border);

        if side == *border {
            return Face::Border;
        }

        match props {
            Some(Props {
                kind: Sides::Border(a, b, c),
                ..
            }) => Sides::get_face_border(side, offset, a, b, c),
            None => Face::None,
            _ => unreachable!(),
        }
    }

    fn get_face_full(side: Border, props: &Option<Props>, compass: &Option<Compass>) -> Face {
        match compass {
            Some(compass) => match props {
                Some(Props {
                    kind: Sides::Full(a, b, c, d),
                    ..
                }) => Sides::get_face_full(side, compass, a, b, c, d),
                _ => panic!("this full cell does not contain the same type of piece"),
            },
            None => match props {
                None => Face::None,
                _ => panic!("this cell cannot have a compass without a piece"),
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
    use self::Border::{East, North, South, West};

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
