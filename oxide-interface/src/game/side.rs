use interface::game::Side;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OxideSide {
    White,
    Black,
}

impl Display for OxideSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            OxideSide::White => write!(f, "w"),
            OxideSide::Black => write!(f, "b"),
        }
    }
}

impl const Side for OxideSide {
    const SIDES: [Self; 2] = [Self::White, Self::Black];
    const WHITE: Self = Self::White;
    const BLACK: Self = Self::Black;

    #[inline]
    fn opposite_side(&self) -> Self {
        match self {
            OxideSide::White => OxideSide::Black,
            OxideSide::Black => OxideSide::White,
        }
    }
    #[inline]
    fn is_white(&self) -> bool {
        match self {
            OxideSide::White => true,
            OxideSide::Black => false,
        }
    }
    #[inline]
    fn is_black(&self) -> bool {
        !self.is_white()
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn is_white_works() {
        assert_eq!(<OxideSide as Side>::is_white(&OxideSide::White), true);
        assert_eq!(<OxideSide as Side>::is_white(&OxideSide::Black), false);
    }

    #[test]
    fn is_black_works() {
        assert_eq!(<OxideSide as Side>::is_black(&OxideSide::White), false);
        assert_eq!(<OxideSide as Side>::is_black(&OxideSide::Black), true);
    }

    #[test]
    fn switch_sides_works() {
        assert_eq!(<OxideSide as Side>::opposite_side(&OxideSide::White), OxideSide::Black);
        assert_eq!(<OxideSide as Side>::opposite_side(&OxideSide::Black), OxideSide::White);
    }

    #[test]
    fn display_works() {
        assert_eq!(format!("{}", OxideSide::White), String::from("w"));
        assert_eq!(format!("{}", OxideSide::Black), String::from("b"));
    }
}