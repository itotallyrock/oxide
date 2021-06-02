use crate::game::Position;
use std::fmt::{Debug, Display};

/// A board piece without side (ie. rook, bishop, queen, etc)
pub trait Piece<P: Position>: Sized + Debug + Display + Default + Eq + PartialEq + From<char> {
    /// List of pieces (excluding empty) for iteration
    const PIECES: [Self; 6];
    /// Piece representing a pawn
    const PAWN: Self;
    /// Piece representing a knight
    const KNIGHT: Self;
    /// Piece representing a bishop
    const BISHOP: Self;
    /// Piece representing a rook
    const ROOK: Self;
    /// Piece representing a queen
    const QUEEN: Self;
    /// Piece representing a king
    const KING: Self;
    /// Piece representing an empty square
    const EMPTY: Self;

    /// Get the sided piece version of a piece for a given side
    fn add_side(self, side: P::Side) -> P::SidedPiece;
}

/// A side's piece (ie. white rook)
pub trait SidedPiece<P: Position>: Sized + Debug + Display + Default + Eq + PartialEq + From<char> + Into<char> {
    /// List of aLl pieces (excluding empty) for iteration
    const PIECES: [Self; 12];
    /// Piece representing a white pawn
    const WHITE_PAWN: Self;
    /// Piece representing a black pawn
    const BLACK_PAWN: Self;
    /// Piece representing a white knight
    const WHITE_KNIGHT: Self;
    /// Piece representing a black knight
    const BLACK_KNIGHT: Self;
    /// Piece representing a white bishop
    const WHITE_BISHOP: Self;
    /// Piece representing a black bishop
    const BLACK_BISHOP: Self;
    /// Piece representing a white rook
    const WHITE_ROOK: Self;
    /// Piece representing a black rook
    const BLACK_ROOK: Self;
    /// Piece representing a white queen
    const WHITE_QUEEN: Self;
    /// Piece representing a black queen
    const BLACK_QUEEN: Self;
    /// Piece for white's king
    const WHITE_KING: Self;
    /// Piece for black's king
    const BLACK_KING: Self;
    /// Piece for no piece (empty square on board)
    const EMPTY: Self;

    /// Get the side of a sided piece
    fn side(&self) -> P::Side;
    /// Get the unsided-piece (ie white-pawn -> pawn, black-rook -> rook)
    fn unsided_piece(&self) -> P::Piece;
}
