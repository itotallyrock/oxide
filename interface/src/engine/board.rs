use crate::game::{Position, SimpleChessMove};
use crate::types::PlyCount;
use crate::game::ChessMove;
use std::hash::Hasher;
use std::error::Error;

/// Computed board state that can be cached between moves to avoid expensive re-computation
pub trait CachedBoardState<P: Position>: Sized {
    /// Mask for a side's pinning pieces
    fn pinning_mask(&self, side: P::Side) -> P::BoardMask;
    /// Mask for a side's pin blocking pieces
    fn blocking_mask(&self, side: P::Side) -> P::BoardMask;
    /// Mask of pieces attacking the side to move's king square
    fn checkers_mask(&self) -> P::BoardMask;
    /// Mask for all squares a given piece for the side to move can give check from
    fn piece_check_mask(&self, piece: P::Piece) -> P::BoardMask;
}

/// Board state that can't be maintained across moves (required to undo a move)
pub trait IdempotentBoardState<P: Position>: Sized {
    type BoardHasher: Hasher;

    /// The castle rights from the previous board
    fn castle_rights(&self) -> P::CastleRights;
    /// The current en-passant square if any
    fn en_passant_square(&self) -> Option<P::Square>;
    /// The piece that was captured last move (Empty piece if none)
    fn captured_piece(&self) -> P::Piece;
    /// The half move clock from the last move
    fn halfmove_clock(&self) -> PlyCount;
    /// A copy of the board hasher for the current position
    fn hasher(&self) -> Self::BoardHasher;
}

/// Board state conglomerate of computed/cached data and required state for manipulating the board though moves
pub trait BoardState<P: Position>: IdempotentBoardState<P> + CachedBoardState<P> {
    fn new(position: &P) -> Self;
}

pub trait Board<P: Position> {
    type BoardState: BoardState<P>;
    type SimpleMove: SimpleChessMove<P>;
    type Move: ChessMove<P>;
    type IllegalMoveError: Error;
    type UndoMoveError: Error;

    /// Create a board from a position
    fn new(position: P) -> Self;
    /// Get the current state of the board
    fn state(&self) -> &Self::BoardState;
    /// Get the current position of the board
    fn position(&self) -> &P;
    /// Make a move on the board and return the required state to undo
    fn make_move(&mut self, chess_move: Self::Move) -> Result<Self::BoardState, Self::IllegalMoveError>;
    /// Make a known legal move on the board (useful with a move gen that only gives legal moves)
    /// Undefined behavior (could panic or continue in undefined state) with illegal move
    fn make_move_unchecked(&mut self, chess_move: Self::Move) -> Self::BoardState;
    /// Undo a previously made move on the board given a previous state
    fn undo_move(&mut self, chess_move: Self::Move, previous_state: Self::BoardState) -> Result<(), Self::UndoMoveError>;
    /// Undo a previously made move on the board given a previous state
    /// Undefined behavior (could panic or continue in undefined state) with illegal move
    fn undo_move_unchecked(&mut self, chess_move: Self::Move, previous_state: Self::BoardState);

    /// If the side to move is in check
    fn in_check(&self) -> bool;
    /// If a given move is a discovery check
    fn is_discovery(&self, chess_move: &Self::Move) -> bool;
    /// If a given move gives check directly
    fn gives_check(&self, chess_move: &Self::Move) -> bool;
    /// If a given move is legal given the current board configuration
    fn is_legal(&self, chess_move: &Self::Move) -> bool;
}