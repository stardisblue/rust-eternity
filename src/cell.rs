use board::Compass;
use piece::{Props, Sides};

#[derive(Debug, PartialEq)]
pub enum Cell {
    CornerCell(Option<Props>, (Border, Border)),
    BorderCell(Option<Props>, Border),
    FullCell(Option<Props>, Option<Compass>),
}

impl Cell {
    // todo: test
    pub fn get_compass(&self) -> Option<Compass> {
        match self {
            Cell::CornerCell(_, borders) => Some(Cell::get_corner_offset(borders)),
            Cell::BorderCell(_, border) => Some(Cell::get_border_offset(border)),
            Cell::FullCell(Some(_), Some(compass)) => Some(compass.clone()),
            Cell::FullCell(None, None) => None,
            _ => panic!("cannot retrieve compass"),
        }
    }

    // todo: test
    pub fn get_corner_offset(borders: &(Border, Border)) -> Compass {
        match borders {
            (Border::North, Border::West) => Compass::East,
            (Border::North, Border::East) => Compass::South,
            (Border::South, Border::East) => Compass::West,
            (Border::South, Border::West) => Compass::North,
            _ => panic!("Not correct borders for corner"),
        }
    }

    // todo: test
    pub fn get_border_offset(border: &Border) -> Compass {
        match border {
            Border::North => Compass::East,
            Border::East => Compass::South,
            Border::South => Compass::West,
            Border::West => Compass::North,
        }
    }

    pub fn get_faces(&self) -> (Face, Face, Face, Face) {
        match self {
            Cell::CornerCell(props, borders) => Cell::get_faces_corner(props, borders),
            Cell::BorderCell(props, border) => Cell::get_faces_border(props, border),
            Cell::FullCell(props, compass) => Cell::get_faces_full(props, compass),
        }
    }

    fn get_faces_corner(pps: &Option<Props>, brds: &(Border, Border)) -> (Face, Face, Face, Face) {
        use self::Border::{East, North, South, West};

        let compass = Sides::get_corner_offset(brds);

        match pps {
            Some(Props {
                kind: Sides::Corner(a, b),
                ..
            }) => Sides::get_faces_corner(&compass, a, b),
            None => {
                let (ns, we) = brds;
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
            }) => Sides::get_faces_border(&compass, a, b, c),
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

    fn get_faces_full(pps: &Option<Props>, cps: &Option<Compass>) -> (Face, Face, Face, Face) {
        match cps {
            Some(compass) => match pps {
                Some(Props {
                    kind: Sides::Full(a, b, c, d),
                    ..
                }) => Sides::get_faces_full(compass, a, b, c, d),
                _ => panic!("Not a full piece"),
            },
            None => match pps {
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
            }) => Sides::get_face_corner(side, &offset, a, b),
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
            }) => Sides::get_face_border(side, &offset, a, b, c),
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

#[derive(Debug, PartialEq)]
pub enum Face {
    Border,
    None,
    Color(u8),
}

#[cfg(test)]
mod tests {
    use self::Border::{East, North, South, West};
    use super::*;

    #[test]
    fn test_get_faces_corner() {
        assert_eq!(
            Cell::CornerCell(None, (North, West)).get_faces(),
            (Face::Border, Face::None, Face::None, Face::Border)
        );
        assert_eq!(
            Cell::CornerCell(None, (North, East)).get_faces(),
            (Face::Border, Face::Border, Face::None, Face::None)
        );
        assert_eq!(
            Cell::CornerCell(None, (South, East)).get_faces(),
            (Face::None, Face::Border, Face::Border, Face::None,)
        );
        assert_eq!(
            Cell::CornerCell(None, (South, West)).get_faces(),
            (Face::None, Face::None, Face::Border, Face::Border)
        );

        let props = Some(Props {
            id: 0,
            kind: Sides::Corner(1, 2),
        });

        assert_eq!(
            Cell::CornerCell(props, (North, West),).get_faces(),
            (Face::Border, Face::Color(1), Face::Color(2), Face::Border)
        );
        assert_eq!(
            Cell::CornerCell(props, (North, East),).get_faces(),
            (Face::Border, Face::Border, Face::Color(1), Face::Color(2))
        );
        assert_eq!(
            Cell::CornerCell(props, (South, East),).get_faces(),
            (Face::Color(2), Face::Border, Face::Border, Face::Color(1))
        );
        assert_eq!(
            Cell::CornerCell(props, (South, West)).get_faces(),
            (Face::Color(1), Face::Color(2), Face::Border, Face::Border)
        );
    }
    #[test]
    fn test_get_faces_border() {
        assert_eq!(
            Cell::BorderCell(None, North).get_faces(),
            (Face::Border, Face::None, Face::None, Face::None)
        );
        assert_eq!(
            Cell::BorderCell(None, East).get_faces(),
            (Face::None, Face::Border, Face::None, Face::None)
        );
        assert_eq!(
            Cell::BorderCell(None, South).get_faces(),
            (Face::None, Face::None, Face::Border, Face::None,)
        );
        assert_eq!(
            Cell::BorderCell(None, West).get_faces(),
            (Face::None, Face::None, Face::None, Face::Border)
        );

        let props = Some(Props {
            id: 0,
            kind: Sides::Border(1, 2, 3),
        });

        assert_eq!(
            Cell::BorderCell(props, North).get_faces(),
            (Face::Border, Face::Color(1), Face::Color(2), Face::Color(3))
        );
        assert_eq!(
            Cell::BorderCell(props, East).get_faces(),
            (Face::Color(3), Face::Border, Face::Color(1), Face::Color(2))
        );
        assert_eq!(
            Cell::BorderCell(props, South).get_faces(),
            (Face::Color(2), Face::Color(3), Face::Border, Face::Color(1))
        );
        assert_eq!(
            Cell::BorderCell(props, West).get_faces(),
            (Face::Color(1), Face::Color(2), Face::Color(3), Face::Border)
        );
    }

    #[test]
    fn test_get_faces_full() {
        use self::Face::Color;
        assert_eq!(
            Cell::FullCell(None, None).get_faces(),
            (Face::None, Face::None, Face::None, Face::None)
        );

        let props = Some(Props {
            id: 0,
            kind: Sides::Full(1, 2, 3, 4),
        });

        assert_eq!(
            Cell::FullCell(props, Some(Compass::North)).get_faces(),
            (Color(1), Color(2), Color(3), Color(4))
        );
        assert_eq!(
            Cell::FullCell(props, Some(Compass::East)).get_faces(),
            (Color(4), Color(1), Color(2), Color(3))
        );
        assert_eq!(
            Cell::FullCell(props, Some(Compass::South)).get_faces(),
            (Color(3), Color(4), Color(1), Color(2))
        );
        assert_eq!(
            Cell::FullCell(props, Some(Compass::West)).get_faces(),
            (Color(2), Color(3), Color(4), Color(1))
        );
    }

    #[test]
    fn test_get_face_corner() {
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

        let props = Some(Props {
            id: 0,
            kind: Sides::Corner(1, 2),
        });
        let corner_cell = Cell::CornerCell(props, (North, West));
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::Color(1), corner_cell.get_face(East));
        assert_eq!(Face::Color(2), corner_cell.get_face(South));

        let corner_cell = Cell::CornerCell(props, (North, East));
        assert_eq!(Face::Border, corner_cell.get_face(North));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::Color(1), corner_cell.get_face(South));
        assert_eq!(Face::Color(2), corner_cell.get_face(West));

        let corner_cell = Cell::CornerCell(props, (South, West));
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(West));
        assert_eq!(Face::Color(1), corner_cell.get_face(North));
        assert_eq!(Face::Color(2), corner_cell.get_face(East));

        let corner_cell = Cell::CornerCell(props, (South, East));
        assert_eq!(Face::Border, corner_cell.get_face(South));
        assert_eq!(Face::Border, corner_cell.get_face(East));
        assert_eq!(Face::Color(1), corner_cell.get_face(West));
        assert_eq!(Face::Color(2), corner_cell.get_face(North));
    }

    #[test]
    fn test_get_face_border() {
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

        let props = Some(Props {
            id: 4,
            kind: Sides::Border(1, 2, 3),
        });
        let border_cell = Cell::BorderCell(props, North);
        assert_eq!(border_cell.get_face(North), Face::Border);
        assert_eq!(border_cell.get_face(East), Face::Color(1));
        assert_eq!(border_cell.get_face(South), Face::Color(2));
        assert_eq!(border_cell.get_face(West), Face::Color(3));

        let border_cell = Cell::BorderCell(props, East);
        assert_eq!(border_cell.get_face(East), Face::Border);
        assert_eq!(border_cell.get_face(South), Face::Color(1));
        assert_eq!(border_cell.get_face(West), Face::Color(2));
        assert_eq!(border_cell.get_face(North), Face::Color(3));

        let border_cell = Cell::BorderCell(props, South);
        assert_eq!(border_cell.get_face(South), Face::Border);
        assert_eq!(border_cell.get_face(West), Face::Color(1));
        assert_eq!(border_cell.get_face(North), Face::Color(2));
        assert_eq!(border_cell.get_face(East), Face::Color(3));

        let border_cell = Cell::BorderCell(props, West);
        assert_eq!(border_cell.get_face(West), Face::Border);
        assert_eq!(border_cell.get_face(North), Face::Color(1));
        assert_eq!(border_cell.get_face(East), Face::Color(2));
        assert_eq!(border_cell.get_face(South), Face::Color(3));
    }

    #[test]
    fn test_get_face_full() {
        let full_cell = Cell::FullCell(None, None);
        assert_eq!(full_cell.get_face(North), Face::None);
        assert_eq!(full_cell.get_face(East), Face::None);
        assert_eq!(full_cell.get_face(South), Face::None);
        assert_eq!(full_cell.get_face(West), Face::None);

        let props = Some(Props {
            id: 12,
            kind: Sides::Full(1, 2, 3, 4),
        });
        let full_cell = Cell::FullCell(props, Some(Compass::North));
        assert_eq!(full_cell.get_face(North), Face::Color(1));
        assert_eq!(full_cell.get_face(East), Face::Color(2));
        assert_eq!(full_cell.get_face(South), Face::Color(3));
        assert_eq!(full_cell.get_face(West), Face::Color(4));

        let full_cell = Cell::FullCell(props, Some(Compass::East));
        assert_eq!(full_cell.get_face(East), Face::Color(1));
        assert_eq!(full_cell.get_face(South), Face::Color(2));
        assert_eq!(full_cell.get_face(West), Face::Color(3));
        assert_eq!(full_cell.get_face(North), Face::Color(4));

        let full_cell = Cell::FullCell(props, Some(Compass::South));
        assert_eq!(full_cell.get_face(South), Face::Color(1));
        assert_eq!(full_cell.get_face(West), Face::Color(2));
        assert_eq!(full_cell.get_face(North), Face::Color(3));
        assert_eq!(full_cell.get_face(East), Face::Color(4));

        let full_cell = Cell::FullCell(props, Some(Compass::West));
        assert_eq!(full_cell.get_face(West), Face::Color(1));
        assert_eq!(full_cell.get_face(North), Face::Color(2));
        assert_eq!(full_cell.get_face(East), Face::Color(3));
        assert_eq!(full_cell.get_face(South), Face::Color(4));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_get_face_corner_piece() {
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
    #[should_panic(expected = "Not correct borders for corner")]
    fn test_get_face_corner_border() {
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
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_get_face_border_piece() {
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
    #[should_panic(expected = "this full cell does not contain the same type of piece")]
    fn test_get_face_full_piece_panic() {
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
    #[should_panic(expected = "this cell cannot have a compass without a piece")]
    fn test_get_face_full_compass_missing_panic() {
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
    #[should_panic(expected = "this full cell does not contain the same type of piece")]
    fn test_get_face_full_no_piece_compass_panic() {
        let full_cell = Cell::FullCell(None, Some(Compass::North));
        full_cell.get_face(South);
    }
}
