use std::fmt::{Debug, Display};
use crate::game::{Piece, Side, Square, BoardMask};

pub trait SimpleChessMove<BitboardType: BoardMask, SquareType: Square<BitboardType, 64>>: Display + Debug + Sized + Clone {
    fn new(from: SquareType, to: SquareType) -> Self;
    fn from(&self) -> SquareType;
    fn to(&self) -> SquareType;
}

pub trait ChessMove<SideType: Side, PieceType: Piece<SideType>, BitboardType: BoardMask, SquareType: Square<BitboardType, 64>>: SimpleChessMove<BitboardType, SquareType> {
    const WHITE_KING_CASTLE: Self;
    const WHITE_QUEEN_CASTLE: Self;
    const BLACK_KING_CASTLE: Self;
    const BLACK_QUEEN_CASTLE: Self;

    fn promotion(&self) -> PieceType;
    fn is_quiet(&self) -> bool;
    fn is_double_pawn_push(&self) -> bool;
    fn is_promotion(&self) -> bool;
    fn is_capture(&self) -> bool;
    fn is_king_castle(&self) -> bool;
    fn is_queen_castle(&self) -> bool;
    fn is_en_passant_capture(&self) -> bool;
    fn set_capture(&mut self, captures: bool);
    fn set_promotion(&mut self, promotion: PieceType);
}