use crate::game::{Position, SimpleChessMove};
use crate::types::PlyCount;
use crate::game::ChessMove;
use std::hash::Hasher;

/// Board state that can't be maintained across moves (required to undo a move)
pub trait IdempotentBoardState<P: Position>: Sized {
    type BoardHasher: Hasher;

    fn castle_rights(&self) -> P::CastleRights;
    fn en_passant_square(&self) -> Option<P::Square>;
    fn captured_piece(&self) -> P::Piece;
    fn halfmove_clock(&self) -> PlyCount;
    fn hasher(&self) -> Self::BoardHasher;
}

pub trait Board: Position {
    type BoardState: IdempotentBoardState<Self>;
    type SimpleMove: SimpleChessMove<Self>;
    type Move: ChessMove<Self>;

    /// Mask of pieces attacking the side to move's king square
    fn checkers_mask(&self) -> Self::BoardMask;
    /// Mask for all squares a given piece for the side to move can give check from
    fn piece_check_mask(&self, piece: Self::Piece) -> Self::BoardMask;
    /// Make a move on the board and return the required state to undo
    fn make_move(&mut self, chess_move: Self::Move) -> Self::BoardState; // TODO: Use Result here
    /// Undo a previously made move on the board given a previous state
    fn undo_move(&mut self, chess_move: Self::Move, previous_state: Self::BoardState); // TODO: Use Result here
}