use interface::game::{CastleRights, Side, PieceArrangement, BoardMask};
use crate::game::{OxideSide, OxideBitboard};
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::ops::{BitAnd, BitOr, Not};
use std::convert::TryFrom;
use crate::engine::{OxideFenParseError, OxidePosition};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OxideCastleRights {
    None,
    WhiteKing,
    WhiteQueen,
    WhiteAll,
    BlackKing,
    BothKings,
    WhiteQueenBlackKing,
    WhiteAllBlackKing,
    BlackQueen,
    WhiteKingBlackQueen,
    BothQueens,
    WhiteAllBlackQueen,
    BlackAll,
    BlackAllWhiteKing,
    BlackAllWhiteQueen,
    All,
}

impl Display for OxideCastleRights {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match *self {
            Self::WHITE_KING => write!(f, "K"),
            Self::WHITE_QUEEN => write!(f, "Q"),
            Self::WHITE_ALL => write!(f, "KQ"),
            Self::BLACK_KING => write!(f, "k"),
            Self::BOTH_KINGS => write!(f, "Kk"),
            Self::WHITE_QUEEN_BLACK_KING => write!(f, "Qk"),
            Self::WHITE_ALL_BLACK_KING => write!(f, "KQk"),
            Self::BLACK_QUEEN => write!(f, "q"),
            Self::WHITE_KING_BLACK_QUEEN => write!(f, "Kq"),
            Self::BOTH_QUEENS => write!(f, "Qq"),
            Self::WHITE_ALL_BLACK_QUEEN => write!(f, "KQq"),
            Self::BLACK_ALL => write!(f, "kq"),
            Self::BLACK_ALL_WHITE_KING => write!(f, "Kkq"),
            Self::BLACK_ALL_WHITE_QUEEN => write!(f, "Qkq"),
            Self::ALL => write!(f, "KQkq"),
            _ => write!(f, "-"),
        }
    }
}

impl CastleRights<OxidePosition> for OxideCastleRights {
    const NONE: Self = Self::None;
    const WHITE_KING: Self = Self::WhiteKing;
    const WHITE_QUEEN: Self = Self::WhiteQueen;
    const WHITE_ALL: Self = Self::WhiteAll;
    const BLACK_KING: Self = Self::BlackKing;
    const BOTH_KINGS: Self = Self::BothKings;
    const WHITE_QUEEN_BLACK_KING: Self = Self::WhiteQueenBlackKing;
    const WHITE_ALL_BLACK_KING: Self = Self::WhiteAllBlackKing;
    const BLACK_QUEEN: Self = Self::BlackQueen;
    const WHITE_KING_BLACK_QUEEN: Self = Self::WhiteKingBlackQueen;
    const BOTH_QUEENS: Self = Self::BothQueens;
    const WHITE_ALL_BLACK_QUEEN: Self = Self::WhiteAllBlackQueen;
    const BLACK_ALL: Self = Self::BlackAll;
    const BLACK_ALL_WHITE_KING: Self = Self::BlackAllWhiteKing;
    const BLACK_ALL_WHITE_QUEEN: Self = Self::BlackAllWhiteQueen;
    const ALL: Self = Self::All;
    #[inline]
    fn for_side(&self, side: OxideSide) -> Self {
        let is_white = side.is_white();
        match self {
            OxideCastleRights::None => OxideCastleRights::None,
            OxideCastleRights::WhiteKing if is_white => OxideCastleRights::WhiteKing,
            OxideCastleRights::WhiteQueen if is_white => OxideCastleRights::WhiteQueen,
            OxideCastleRights::WhiteAll if is_white => OxideCastleRights::WhiteAll,
            OxideCastleRights::BlackKing if !is_white => OxideCastleRights::BlackKing,
            OxideCastleRights::BothKings if is_white => OxideCastleRights::WhiteKing,
            OxideCastleRights::BothKings if !is_white => OxideCastleRights::BlackKing,
            OxideCastleRights::WhiteQueenBlackKing if is_white => OxideCastleRights::WhiteQueen,
            OxideCastleRights::WhiteQueenBlackKing if !is_white => OxideCastleRights::BlackKing,
            OxideCastleRights::WhiteAllBlackKing if is_white => OxideCastleRights::WhiteAll,
            OxideCastleRights::WhiteAllBlackKing if !is_white => OxideCastleRights::BlackKing,
            OxideCastleRights::BlackQueen if !is_white => OxideCastleRights::BlackQueen,
            OxideCastleRights::WhiteKingBlackQueen if is_white => OxideCastleRights::WhiteKing,
            OxideCastleRights::WhiteKingBlackQueen if !is_white => OxideCastleRights::BlackQueen,
            OxideCastleRights::BothQueens if is_white => OxideCastleRights::WhiteQueen,
            OxideCastleRights::BothQueens if !is_white => OxideCastleRights::BlackQueen,
            OxideCastleRights::WhiteAllBlackQueen if is_white => OxideCastleRights::WhiteAll,
            OxideCastleRights::WhiteAllBlackQueen if !is_white => OxideCastleRights::BlackQueen,
            OxideCastleRights::BlackAll if !is_white => OxideCastleRights::BlackAll,
            OxideCastleRights::BlackAllWhiteKing if is_white => OxideCastleRights::WhiteKing,
            OxideCastleRights::BlackAllWhiteKing if !is_white => OxideCastleRights::BlackAll,
            OxideCastleRights::BlackAllWhiteQueen if is_white => OxideCastleRights::WhiteQueen,
            OxideCastleRights::BlackAllWhiteQueen if !is_white => OxideCastleRights::BlackAll,
            OxideCastleRights::All if is_white => OxideCastleRights::WhiteAll,
            OxideCastleRights::All if !is_white => OxideCastleRights::BlackAll,
            _ => OxideCastleRights::None,
        }
    }
    #[inline]
    fn contains(&self, other: Self) -> bool {
        *self & other == other
    }
    #[inline]
    fn intersects(&self, other: Self) -> bool {
        (*self & other) as u8 > 0
    }
    #[inline]
    fn insert(&mut self, other: Self) {
        *self = *self | other;
    }
    #[inline]
    fn remove(&mut self, other: Self) {
        *self = *self & !other
    }

    fn castle_path(&self) -> OxideBitboard {
        const WHITE_KING_CLEAR: OxideBitboard = OxideBitboard(0x60);
        const WHITE_QUEEN_CLEAR: OxideBitboard = OxideBitboard(0xE);
        const BLACK_KING_CLEAR: OxideBitboard = OxideBitboard(0x6000000000000000);
        const BLACK_QUEEN_CLEAR: OxideBitboard = OxideBitboard(0xe00000000000000);
        match *self {
            OxideCastleRights::NONE => OxideBitboard::EMPTY,
            OxideCastleRights::WHITE_KING => WHITE_KING_CLEAR,
            OxideCastleRights::WHITE_QUEEN => WHITE_QUEEN_CLEAR,
            OxideCastleRights::WHITE_ALL => WHITE_KING_CLEAR | WHITE_QUEEN_CLEAR,
            OxideCastleRights::BLACK_KING => BLACK_KING_CLEAR,
            OxideCastleRights::BOTH_KINGS => WHITE_KING_CLEAR | BLACK_KING_CLEAR,
            OxideCastleRights::WHITE_QUEEN_BLACK_KING => WHITE_QUEEN_CLEAR | BLACK_KING_CLEAR,
            OxideCastleRights::WHITE_ALL_BLACK_KING => WHITE_QUEEN_CLEAR | WHITE_KING_CLEAR | BLACK_KING_CLEAR,
            OxideCastleRights::BLACK_QUEEN => BLACK_QUEEN_CLEAR,
            OxideCastleRights::WHITE_KING_BLACK_QUEEN => WHITE_KING_CLEAR | BLACK_QUEEN_CLEAR,
            OxideCastleRights::BOTH_QUEENS => WHITE_QUEEN_CLEAR | BLACK_QUEEN_CLEAR,
            OxideCastleRights::WHITE_ALL_BLACK_QUEEN => WHITE_QUEEN_CLEAR | WHITE_KING_CLEAR | BLACK_QUEEN_CLEAR,
            OxideCastleRights::BLACK_ALL => BLACK_QUEEN_CLEAR | BLACK_KING_CLEAR,
            OxideCastleRights::BLACK_ALL_WHITE_KING => BLACK_QUEEN_CLEAR | BLACK_KING_CLEAR | WHITE_KING_CLEAR,
            OxideCastleRights::BLACK_ALL_WHITE_QUEEN => BLACK_QUEEN_CLEAR | BLACK_KING_CLEAR | WHITE_QUEEN_CLEAR,
            OxideCastleRights::ALL => BLACK_QUEEN_CLEAR | BLACK_KING_CLEAR | WHITE_KING_CLEAR | WHITE_QUEEN_CLEAR,
        }
    }
}

impl const BitOr for OxideCastleRights {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let or_num = (self as u8) | (rhs as u8);

        unsafe { std::mem::transmute(or_num) }
    }
}

impl const BitAnd for OxideCastleRights {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let and_num = (self as u8) & (rhs as u8);

        unsafe { std::mem::transmute(and_num) }
    }
}

impl const Not for OxideCastleRights {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        unsafe { std::mem::transmute(!(self as u8)) }
    }
}

impl TryFrom<&str> for OxideCastleRights {
    type Error = OxideFenParseError;

    fn try_from(castle_str: &str) -> Result<Self, Self::Error> {
        let mut rights = OxideCastleRights::NONE;
        for c in castle_str.chars().take(4) {
            match c {
                'K' => rights = rights | OxideCastleRights::WHITE_KING,
                'k' => rights = rights | OxideCastleRights::BLACK_KING,
                'Q' => rights = rights | OxideCastleRights::WHITE_QUEEN,
                'q' => rights = rights | OxideCastleRights::BLACK_QUEEN,
                '-' if rights == OxideCastleRights::NONE => break,
                _ => return Err(OxideFenParseError::CastleParseError),
            }
        }

        Ok(rights)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_side_works() {
        assert_eq!(OxideCastleRights::None.for_side(OxideSide::White), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::None.for_side(OxideSide::Black), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::WhiteKing.for_side(OxideSide::White), OxideCastleRights::WhiteKing);
        assert_eq!(OxideCastleRights::WhiteKing.for_side(OxideSide::Black), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::WhiteQueen.for_side(OxideSide::White), OxideCastleRights::WhiteQueen);
        assert_eq!(OxideCastleRights::WhiteQueen.for_side(OxideSide::Black), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::WhiteAll.for_side(OxideSide::White), OxideCastleRights::WhiteAll);
        assert_eq!(OxideCastleRights::WhiteAll.for_side(OxideSide::Black), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::BlackKing.for_side(OxideSide::White), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::BlackKing.for_side(OxideSide::Black), OxideCastleRights::BlackKing);
        assert_eq!(OxideCastleRights::BothKings.for_side(OxideSide::White), OxideCastleRights::WhiteKing);
        assert_eq!(OxideCastleRights::BothKings.for_side(OxideSide::Black), OxideCastleRights::BlackKing);
        assert_eq!(OxideCastleRights::WhiteQueenBlackKing.for_side(OxideSide::White), OxideCastleRights::WhiteQueen);
        assert_eq!(OxideCastleRights::WhiteQueenBlackKing.for_side(OxideSide::Black), OxideCastleRights::BlackKing);
        assert_eq!(OxideCastleRights::WhiteAllBlackKing.for_side(OxideSide::White), OxideCastleRights::WhiteAll);
        assert_eq!(OxideCastleRights::WhiteAllBlackKing.for_side(OxideSide::Black), OxideCastleRights::BlackKing);
        assert_eq!(OxideCastleRights::BlackQueen.for_side(OxideSide::White), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::BlackQueen.for_side(OxideSide::Black), OxideCastleRights::BlackQueen);
        assert_eq!(OxideCastleRights::WhiteKingBlackQueen.for_side(OxideSide::White), OxideCastleRights::WhiteKing);
        assert_eq!(OxideCastleRights::WhiteKingBlackQueen.for_side(OxideSide::Black), OxideCastleRights::BlackQueen);
        assert_eq!(OxideCastleRights::BothQueens.for_side(OxideSide::White), OxideCastleRights::WhiteQueen);
        assert_eq!(OxideCastleRights::BothQueens.for_side(OxideSide::Black), OxideCastleRights::BlackQueen);
        assert_eq!(OxideCastleRights::WhiteAllBlackQueen.for_side(OxideSide::White), OxideCastleRights::WhiteAll);
        assert_eq!(OxideCastleRights::WhiteAllBlackQueen.for_side(OxideSide::Black), OxideCastleRights::BlackQueen);
        assert_eq!(OxideCastleRights::BlackAll.for_side(OxideSide::White), OxideCastleRights::None);
        assert_eq!(OxideCastleRights::BlackAll.for_side(OxideSide::Black), OxideCastleRights::BlackAll);
        assert_eq!(OxideCastleRights::BlackAllWhiteKing.for_side(OxideSide::White), OxideCastleRights::WhiteKing);
        assert_eq!(OxideCastleRights::BlackAllWhiteKing.for_side(OxideSide::Black), OxideCastleRights::BlackAll);
        assert_eq!(OxideCastleRights::BlackAllWhiteQueen.for_side(OxideSide::White), OxideCastleRights::WhiteQueen);
        assert_eq!(OxideCastleRights::BlackAllWhiteQueen.for_side(OxideSide::Black), OxideCastleRights::BlackAll);
        assert_eq!(OxideCastleRights::All.for_side(OxideSide::White), OxideCastleRights::WhiteAll);
        assert_eq!(OxideCastleRights::All.for_side(OxideSide::Black), OxideCastleRights::BlackAll);
    }

    #[test]
    fn try_from_str_works() {
        assert_eq!(OxideCastleRights::try_from("-"), Ok(OxideCastleRights::None));
        assert_eq!(OxideCastleRights::try_from("K"), Ok(OxideCastleRights::WhiteKing));
        assert_eq!(OxideCastleRights::try_from("Q"), Ok(OxideCastleRights::WhiteQueen));
        assert_eq!(OxideCastleRights::try_from("KQ"), Ok(OxideCastleRights::WhiteAll));
        assert_eq!(OxideCastleRights::try_from("k"), Ok(OxideCastleRights::BlackKing));
        assert_eq!(OxideCastleRights::try_from("Kk"), Ok(OxideCastleRights::BothKings));
        assert_eq!(OxideCastleRights::try_from("Qk"), Ok(OxideCastleRights::WhiteQueenBlackKing));
        assert_eq!(OxideCastleRights::try_from("KQk"), Ok(OxideCastleRights::WhiteAllBlackKing));
        assert_eq!(OxideCastleRights::try_from("q"), Ok(OxideCastleRights::BlackQueen));
        assert_eq!(OxideCastleRights::try_from("Kq"), Ok(OxideCastleRights::WhiteKingBlackQueen));
        assert_eq!(OxideCastleRights::try_from("Qq"), Ok(OxideCastleRights::BothQueens));
        assert_eq!(OxideCastleRights::try_from("KQq"), Ok(OxideCastleRights::WhiteAllBlackQueen));
        assert_eq!(OxideCastleRights::try_from("kq"), Ok(OxideCastleRights::BlackAll));
        assert_eq!(OxideCastleRights::try_from("Kkq"), Ok(OxideCastleRights::BlackAllWhiteKing));
        assert_eq!(OxideCastleRights::try_from("Qkq"), Ok(OxideCastleRights::BlackAllWhiteQueen));
        assert_eq!(OxideCastleRights::try_from("KQkq"), Ok(OxideCastleRights::All));
        assert_eq!(OxideCastleRights::try_from("?"), Err(OxideFenParseError::CastleParseError));
    }

    #[test]
    fn display_works() {
        assert_eq!(format!("{}", OxideCastleRights::None), "-");
        assert_eq!(format!("{}", OxideCastleRights::WhiteKing), "K");
        assert_eq!(format!("{}", OxideCastleRights::WhiteQueen), "Q");
        assert_eq!(format!("{}", OxideCastleRights::WhiteAll), "KQ");
        assert_eq!(format!("{}", OxideCastleRights::BlackKing), "k");
        assert_eq!(format!("{}", OxideCastleRights::BothKings), "Kk");
        assert_eq!(format!("{}", OxideCastleRights::WhiteQueenBlackKing), "Qk");
        assert_eq!(format!("{}", OxideCastleRights::WhiteAllBlackKing), "KQk");
        assert_eq!(format!("{}", OxideCastleRights::BlackQueen), "q");
        assert_eq!(format!("{}", OxideCastleRights::WhiteKingBlackQueen), "Kq");
        assert_eq!(format!("{}", OxideCastleRights::BothQueens), "Qq");
        assert_eq!(format!("{}", OxideCastleRights::WhiteAllBlackQueen), "KQq");
        assert_eq!(format!("{}", OxideCastleRights::BlackAll), "kq");
        assert_eq!(format!("{}", OxideCastleRights::BlackAllWhiteKing), "Kkq");
        assert_eq!(format!("{}", OxideCastleRights::BlackAllWhiteQueen), "Qkq");
        assert_eq!(format!("{}", OxideCastleRights::All), "KQkq");
    }
}