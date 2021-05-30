use std::fmt::{Debug, Display};
use crate::game::Position;

pub trait SimpleChessMove<P: Position>: Display + Debug + Sized + Clone {
    fn new(from: P::Square, to: P::Square) -> Self;
    fn from(&self) -> P::Square;
    fn to(&self) -> P::Square;
}

pub trait ChessMove<P: Position>: SimpleChessMove<P> {
    type SimpleChessMove: SimpleChessMove<P>;

    const WHITE_KING_CASTLE: Self;
    const WHITE_QUEEN_CASTLE: Self;
    const BLACK_KING_CASTLE: Self;
    const BLACK_QUEEN_CASTLE: Self;

    fn from_simple_move(simple_move: Self::SimpleChessMove) -> Self;
    fn simple_move(&self) -> Self::SimpleChessMove;
    fn promotion(&self) -> P::Piece;
    fn is_quiet(&self) -> bool;
    fn is_double_pawn_push(&self) -> bool;
    fn is_promotion(&self) -> bool;
    fn is_capture(&self) -> bool;
    fn is_king_castle(&self) -> bool;
    fn is_queen_castle(&self) -> bool;
    fn is_en_passant_capture(&self) -> bool;
    fn set_capture(&mut self);
    fn set_promotion(&mut self, promotion: P::Piece);
}