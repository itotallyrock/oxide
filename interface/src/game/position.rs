use crate::game::{Side, Piece, BoardMask, SidedPiece, Square, CastleRights};
use crate::types::PlyCount;
use std::fmt::Debug;
use std::hash::Hash;

/// Arrangement of pieces on a board and who they belong to
pub trait PieceArrangement<P: Position>: Hash {
    /// Side representation (ie. white or black)
    type Side: Side;
    /// Piece representation (ie. Pawn, Rook, Knight, Queen, Bishop, King, Empty)
    type Piece: Piece<P>;
    /// Piece representation for a specific side (ie. White Pawn, Black Rook, etc)
    type SidedPiece: SidedPiece<P>;
    /// Board mask representation (a set of squares on the board typically a bitboard or list of squares)
    type BoardMask: BoardMask<P>;
    /// Square representation (ie. A3, E4, H8, etc)
    type Square: Square<P>;

    /// Empty position (no pieces on the board)
    const EMPTY: Self;
    /// Board mask representing squares with a given piece (both sides) on it
    fn piece_mask(&self, piece: P::Piece) -> P::BoardMask;
    /// Board mask representing squares with a side's given piece on it
    fn sided_piece_mask(&self, sided_piece: P::SidedPiece) -> P::BoardMask;
    /// Board mask of all squares with a piece on them
    fn occupied(&self) -> P::BoardMask;
    /// Board mask of all squares without a piece on them
    fn empty(&self) -> P::BoardMask;
    /// Board mask representing all of side's pieces
    fn mask_for_side(&self, side: P::Side) -> P::BoardMask;
    /// Get the piece standing on a specific square (Empty if no piece)
    fn piece_on_square(&self, square: P::Square) -> P::Piece;
    /// Get the side if any that for a specific square (None for empty square)
    fn side_on_square(&self, square: P::Square) -> Option<P::Side>;
    /// Get a side's king square
    fn king_square(&self, side: P::Side) -> P::Square;
    // Mutation
    /// Add a side's piece to a specific square
    fn add_piece(&mut self, piece: P::SidedPiece, to_square: P::Square);
    /// Remove a side's piece from a specific square
    fn remove_piece(&mut self, piece: P::SidedPiece, from_square: P::Square);
    /// Move a side's piece from one square to another
    fn move_piece(&mut self, piece: P::SidedPiece, to_square: P::Square, from_square: P::Square);
}

/// A chess board position representing all the pieces, castle permissions, who's turn it is, halfmove clock, and en-passant square
pub trait Position: Sized + Clone + PieceArrangement<Self> {
    /// Castle rights representation (ie. KQkq, Kq, or - from FEN)
    type CastleRights: CastleRights<Self>;
    /// Issues arising from parsing a FEN string (empty string/wrong format, illegal position, etc)
    type FenParseError: Debug;

    // Getters
    /// Attempt to parse a position from a chess FEN string (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
    fn from_fen(fen: &str) -> Result<Self, Self::FenParseError>;
    /// Get the FEN representation of the current position (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
    fn to_fen(&self) -> String;
    /// Get the side (white or black) for who will make the next move
    fn side_to_move(&self) -> Self::Side;
    /// Get the current castle rights for the position (if either white and black has either king or queen-side castle)
    fn castle_rights(&self) -> Self::CastleRights;
    /// Get the en-passant square if a pawn had previously double-jumped
    fn en_passant_square(&self) -> Option<Self::Square>;
    /// Get the halfmove clock (how many moves since a pawn push or capture)
    fn halfmove_clock(&self) -> PlyCount;
    /// Get the fullmove count (number of times both white and black made a moth)
    fn fullmove_count(&self) -> PlyCount;
    // Mutation
    /// Set the boards castle rights
    fn set_castle_rights(&mut self, castle_rights: Self::CastleRights);
    /// Add castle rights for position (adds missing castle rights but leave existing rights alone)
    fn add_castle_rights(&mut self, castle_rights: Self::CastleRights);
    /// Remove castle rights from a position (keeps rights not being removed)
    fn remove_castle_rights(&mut self, castle_rights: Self::CastleRights);
    /// Set the en passant square to a square (the square the pawn jumped)
    fn set_en_passant(&mut self, en_passant_square: Self::Square);
    /// Clear the en passant square
    fn clear_en_passant(&mut self);
    /// Switch which side is moving
    fn switch_sides(&mut self);
    /// Reset the halfmove clock (for pawn push or capture)
    fn reset_halfmove_clock(&mut self);
    /// Increment the halfmove clock (non-pawn pushes and non-captures)
    fn increment_halfmove_clock(&mut self);
}