use crate::game::Position;
use std::fmt::{Debug, Display};

pub trait Piece<P: Position>: Sized + Debug + Display + Default + Eq + PartialEq + From<char> {
    const PIECES: [Self; 6];
    const PAWN: Self;
    const KNIGHT: Self;
    const BISHOP: Self;
    const ROOK: Self;
    const QUEEN: Self;
    const KING: Self;
    const EMPTY: Self;

    fn add_side(self, side: P::Side) -> P::SidedPiece;
}

pub trait SidedPiece<P: Position>: Sized + Debug + Display + Default + Eq + PartialEq + From<char> + Into<char> {
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

    fn side(&self) -> P::Side;
    fn unsided_piece(&self) -> P::Piece;
}
