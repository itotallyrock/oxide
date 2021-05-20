use interface::game::{Piece, Side, SidedPiece};
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OxidePiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OxideSidedPiece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
    Empty,
}

impl Default for OxidePiece {
    #[inline]
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for OxidePiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            OxidePiece::Pawn => write!(f, "P"),
            OxidePiece::Knight => write!(f, "N"),
            OxidePiece::Bishop => write!(f, "B"),
            OxidePiece::Rook => write!(f, "R"),
            OxidePiece::Queen => write!(f, "Q"),
            OxidePiece::King => write!(f, "K"),
            OxidePiece::Empty => write!(f, " "),
        }
    }
}

impl<SideType: Side> Piece<SideType> for OxidePiece {
    type SidedPieceType = OxideSidedPiece;
    const PIECES: [Self; 6] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];

    const PAWN: Self = Self::Pawn;
    const KNIGHT: Self = Self::Knight;
    const BISHOP: Self = Self::Bishop;
    const ROOK: Self = Self::Rook;
    const QUEEN: Self = Self::Queen;
    const KING: Self = Self::King;
    const EMPTY: Self = Self::Empty;

    #[inline]
    fn add_side(self, side: SideType) -> Self::SidedPieceType {
        match self {
            Self::Pawn => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_PAWN } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_PAWN },
            Self::Knight => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_KNIGHT } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_KNIGHT },
            Self::Bishop => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_BISHOP } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_BISHOP },
            Self::Rook => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_ROOK } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_ROOK },
            Self::Queen => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_QUEEN } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_QUEEN },
            Self::King => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::WHITE_KING } else { <Self::SidedPieceType as SidedPiece<SideType>>::BLACK_KING },
            Self::Empty => if side.is_white() { <Self::SidedPieceType as SidedPiece<SideType>>::EMPTY } else { <Self::SidedPieceType as SidedPiece<SideType>>::EMPTY },
        }
    }
}

impl Default for OxideSidedPiece {
    #[inline]
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for OxideSidedPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::WhitePawn => write!(f, "P"),
            Self::BlackPawn => write!(f, "p"),
            Self::WhiteKnight => write!(f, "N"),
            Self::BlackKnight => write!(f, "N"),
            Self::WhiteBishop => write!(f, "B"),
            Self::BlackBishop => write!(f, "B"),
            Self::WhiteRook => write!(f, "R"),
            Self::BlackRook => write!(f, "r"),
            Self::WhiteQueen => write!(f, "Q"),
            Self::BlackQueen => write!(f, "q"),
            Self::WhiteKing => write!(f, "K"),
            Self::BlackKing => write!(f, "k"),
            Self::Empty => write!(f, " "),
        }
    }
}

impl<SideType: Side> SidedPiece<SideType> for OxideSidedPiece {
    type PieceType = OxidePiece;
    const PIECES: [Self; 12] = [
        Self::WhitePawn,
        Self::BlackPawn,
        Self::WhiteKnight,
        Self::BlackKnight,
        Self::WhiteBishop,
        Self::BlackBishop,
        Self::WhiteRook,
        Self::BlackRook,
        Self::WhiteQueen,
        Self::BlackQueen,
        Self::WhiteKing,
        Self::BlackKing,
    ];
    const WHITE_PAWN: Self = Self::WhitePawn;
    const BLACK_PAWN: Self = Self::BlackPawn;
    const WHITE_KNIGHT: Self = Self::WhiteKnight;
    const BLACK_KNIGHT: Self = Self::BlackKnight;
    const WHITE_BISHOP: Self = Self::WhiteBishop;
    const BLACK_BISHOP: Self = Self::BlackBishop;
    const WHITE_ROOK: Self = Self::WhiteRook;
    const BLACK_ROOK: Self = Self::BlackRook;
    const WHITE_QUEEN: Self = Self::WhiteQueen;
    const BLACK_QUEEN: Self = Self::BlackQueen;
    const WHITE_KING: Self = Self::WhiteKing;
    const BLACK_KING: Self = Self::BlackKing;
    const EMPTY: Self = Self::Empty;

    fn side(&self) -> SideType {
        match self {
            OxideSidedPiece::WhitePawn | OxideSidedPiece::WhiteKnight | OxideSidedPiece::WhiteBishop | OxideSidedPiece::WhiteRook | OxideSidedPiece::WhiteQueen | OxideSidedPiece::WhiteKing => SideType::WHITE,
            OxideSidedPiece::BlackPawn | OxideSidedPiece::BlackKnight | OxideSidedPiece::BlackBishop | OxideSidedPiece::BlackRook | OxideSidedPiece::BlackQueen | OxideSidedPiece::BlackKing => SideType::BLACK,
            OxideSidedPiece::Empty => panic!("Attempting to get side of none colored piece"),
        }
    }

    fn unsided_piece(&self) -> Self::PieceType {
        match self {
            OxideSidedPiece::WhitePawn | OxideSidedPiece::BlackPawn => Self::PieceType::Pawn,
            OxideSidedPiece::WhiteKnight | OxideSidedPiece::BlackKnight => Self::PieceType::Knight,
            OxideSidedPiece::WhiteBishop | OxideSidedPiece::BlackBishop => Self::PieceType::Bishop,
            OxideSidedPiece::WhiteRook | OxideSidedPiece::BlackRook => Self::PieceType::Rook,
            OxideSidedPiece::WhiteQueen | OxideSidedPiece::BlackQueen => Self::PieceType::Queen,
            OxideSidedPiece::WhiteKing | OxideSidedPiece::BlackKing => Self::PieceType::King,
            OxideSidedPiece::Empty => Self::PieceType::Empty,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::OxideSide;

    #[test]
    fn remove_side_works() {
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhitePawn), OxidePiece::Pawn);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackPawn), OxidePiece::Pawn);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhiteKnight), OxidePiece::Knight);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackKnight), OxidePiece::Knight);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhiteBishop), OxidePiece::Bishop);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackBishop), OxidePiece::Bishop);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhiteRook), OxidePiece::Rook);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackRook), OxidePiece::Rook);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhiteQueen), OxidePiece::Queen);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackQueen), OxidePiece::Queen);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::WhiteKing), OxidePiece::King);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::BlackKing), OxidePiece::King);
        assert_eq!(SidedPiece::<OxideSide>::unsided_piece(&OxideSidedPiece::Empty), OxidePiece::Empty);
    }

    #[test]
    fn side_of_sided_works() {
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhitePawn), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackPawn), OxideSide::Black);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhiteKnight), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackKnight), OxideSide::Black);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhiteBishop), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackBishop), OxideSide::Black);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhiteRook), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackRook), OxideSide::Black);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhiteQueen), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackQueen), OxideSide::Black);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::WhiteKing), OxideSide::White);
        assert_eq!(SidedPiece::<OxideSide>::side(&OxideSidedPiece::BlackKing), OxideSide::Black);
    }

    #[should_panic]
    #[test]
    fn side_of_none_panics() {
        SidedPiece::<OxideSide>::side(&OxideSidedPiece::Empty);
    }

    #[test]
    fn sided_display_works() {
        assert_eq!(format!("{}", OxideSidedPiece::WhitePawn), String::from("P"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackPawn), String::from("p"));
        assert_eq!(format!("{}", OxideSidedPiece::WhiteKnight), String::from("N"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackKnight), String::from("N"));
        assert_eq!(format!("{}", OxideSidedPiece::WhiteBishop), String::from("B"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackBishop), String::from("B"));
        assert_eq!(format!("{}", OxideSidedPiece::WhiteRook), String::from("R"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackRook), String::from("r"));
        assert_eq!(format!("{}", OxideSidedPiece::WhiteQueen), String::from("Q"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackQueen), String::from("q"));
        assert_eq!(format!("{}", OxideSidedPiece::WhiteKing), String::from("K"));
        assert_eq!(format!("{}", OxideSidedPiece::BlackKing), String::from("k"));
        assert_eq!(format!("{}", OxideSidedPiece::Empty), String::from(" "));
    }

    #[test]
    fn unsided_display_works() {
        assert_eq!(format!("{}", OxidePiece::Pawn), String::from("P"));
        assert_eq!(format!("{}", OxidePiece::Knight), String::from("N"));
        assert_eq!(format!("{}", OxidePiece::Bishop), String::from("B"));
        assert_eq!(format!("{}", OxidePiece::Rook), String::from("R"));
        assert_eq!(format!("{}", OxidePiece::Queen), String::from("Q"));
        assert_eq!(format!("{}", OxidePiece::King), String::from("K"));
        assert_eq!(format!("{}", OxidePiece::Empty), String::from(" "));
    }

    #[test]
    fn default_pieces_are_empty() {
        assert_eq!(OxidePiece::default(), OxidePiece::Empty);
        assert_eq!(OxideSidedPiece::default(), OxideSidedPiece::Empty);
    }
}