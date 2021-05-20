use crate::game::Side;
use std::fmt::{Debug, Display};

pub trait Piece<SideType: Side>: Sized + Debug + Display + Default + Eq + PartialEq {
    type SidedPieceType: SidedPiece<SideType>;
    const PIECES: [Self; 6];
    const PAWN: Self;
    const KNIGHT: Self;
    const BISHOP: Self;
    const ROOK: Self;
    const QUEEN: Self;
    const KING: Self;
    const EMPTY: Self;

    fn add_side(self, side: SideType) -> Self::SidedPieceType;
}

pub trait SidedPiece<SideType: Side>:  Sized + Debug + Display + Default + Eq + PartialEq {
    type PieceType: Piece<SideType>;
    const PIECES: [Self; 12];
    const WHITE_PAWN: Self;
    const BLACK_PAWN: Self;
    const WHITE_KNIGHT: Self;
    const BLACK_KNIGHT: Self;
    const WHITE_BISHOP: Self;
    const BLACK_BISHOP: Self;
    const WHITE_ROOK: Self;
    const BLACK_ROOK: Self;
    const WHITE_QUEEN: Self;
    const BLACK_QUEEN: Self;
    const WHITE_KING: Self;
    const BLACK_KING: Self;
    const EMPTY: Self;

    fn side(&self) -> SideType;
    fn remove_side(self) -> Self::PieceType;
}
