use std::fmt::{Debug, Display};
use crate::game::Position;
use crate::engine::Board;

/// Basic mve with only the origin and destination
pub trait SimpleChessMove<P: Position>: Display + Debug + Sized + Clone {
    /// Create a simple move going from one square to another
    fn new(from: P::Square, to: P::Square) -> Self;
    /// Starting square
    fn from(&self) -> P::Square;
    /// Ending square
    fn to(&self) -> P::Square;
}

/// Move with information on its side-effects (thus can be applied to a board)
pub trait ChessMove<P: Position>: SimpleChessMove<P> {
    /// Type for simple move to inherit from
    type SimpleChessMove: SimpleChessMove<P>;
    /// Type for board representation to pull move information from
    type Board: Board<P>;

    /// Special const to avoid re-constructing multiple white king castles
    const WHITE_KING_CASTLE: Self;
    /// Special const to avoid re-constructing multiple white queen castles
    const WHITE_QUEEN_CASTLE: Self;
    /// Special const to avoid re-constructing multiple black king castles
    const BLACK_KING_CASTLE: Self;
    /// Special const to avoid re-constructing multiple black queen castles
    const BLACK_QUEEN_CASTLE: Self;

    /// Create a new double pawn push move
    fn new_double_pawn_push(from: P::Square, to: P::Square) -> Self;
    /// Create a new en passant pawn capture move
    fn new_en_passant_capture(from: P::Square, to: P::Square) -> Self;
    /// Create a new capture move
    fn new_capture(from: P::Square, to: P::Square) -> Self;
    /// Create a new pawn promotion move
    fn new_promotion(from: P::Square, to: P::Square, promotion: P::Piece) -> Self;
    /// Create a new pawn promotion and capture move
    fn new_promoting_capture(from: P::Square, to: P::Square, promotion: P::Piece) -> Self;

    /// Make a chess move from a simple move (determine what side-effects a move has (ie. the move captures or leaves an en-passant square))
    fn from_simple_move(simple_move: Self::SimpleChessMove, board: &Self::Board) -> Self;
    /// Get the promotion piece (empty if non-promoting move)
    fn promotion(&self) -> P::Piece;
    /// If a move is has no side-effects (not a capture, double pawn push, castle, or promotion)
    fn is_quiet(&self) -> bool;
    /// If a move is a double pawn push
    fn is_double_pawn_push(&self) -> bool;
    /// If a move is a promotion
    fn is_promotion(&self) -> bool;
    /// If a move is a capture
    fn is_capture(&self) -> bool;
    /// If a move is a king-side castle
    fn is_king_castle(&self) -> bool;
    /// If a move is a queen-side castle
    fn is_queen_castle(&self) -> bool;
    /// If a move is an en-passant capture
    fn is_en_passant_capture(&self) -> bool;
}