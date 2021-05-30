use crate::game::{Side, Piece, BoardMask, SidedPiece, Square, CastleRights};
use crate::types::PlyCount;
use std::fmt::Debug;
use std::hash::Hash;

pub trait PieceArrangement<P: Position>: Hash {
    type Side: Side;
    type Piece: Piece<P>;
    type SidedPiece: SidedPiece<P>;
    type BoardMask: BoardMask<P>;
    type Square: Square<P>;

    const EMPTY: Self;
    fn piece_mask(&self, piece: P::Piece) -> P::BoardMask;
    fn sided_piece_mask(&self, sided_piece: P::SidedPiece) -> P::BoardMask;
    fn occupied(&self) -> P::BoardMask;
    fn empty(&self) -> P::BoardMask;
    fn piece_mask_for_side(&self, side: P::Side) -> P::BoardMask;
    fn piece_on_square(&self, square: P::Square) -> P::Piece;
    fn side_on_square(&self, square: P::Square) -> Option<P::Side>;
    fn king_square(&self, side: P::Side) -> P::Square;
    // Mutation
    fn add_piece(&mut self, piece: P::SidedPiece, to_square: P::Square);
    fn remove_piece(&mut self, piece: P::SidedPiece, from_square: P::Square);
    fn move_piece(&mut self, piece: P::SidedPiece, to_square: P::Square, from_square: P::Square);
}

pub trait Position: Sized + Clone + PieceArrangement<Self> {
    type CastleRights: CastleRights<Self>;
    type FenParseError: Debug;

    fn from_fen(fen: &str) -> Result<Self, Self::FenParseError>;
    fn to_fen(&self) -> String;
    fn side_to_move(&self) -> Self::Side;
    fn castle_rights(&self) -> Self::CastleRights;
    fn en_passant_square(&self) -> Option<Self::Square>;
    fn halfmove_clock(&self) -> PlyCount;
    fn fullmove_count(&self) -> PlyCount;
    // Mutation
    fn set_castle_rights(&mut self, castle_rights: Self::CastleRights);
    fn add_castle_rights(&mut self, castle_rights: Self::CastleRights);
    fn remove_castle_rights(&mut self, castle_rights: Self::CastleRights);
    fn set_en_passant(&mut self, en_passant_square: Self::Square);
    fn clear_en_passant(&mut self);
    fn switch_sides(&mut self);
    fn reset_halfmove_clock(&mut self);
    fn increment_halfmove_clock(&mut self);
}